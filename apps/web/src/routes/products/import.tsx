import { createFileRoute, Link, useNavigate } from '@tanstack/react-router'
import { useForm } from '@mantine/form'
import {
  Textarea,
  Button,
  Container,
  Title,
  Stack,
  Group,
  Text,
  Paper,
  ThemeIcon,
  Code,
} from '@mantine/core'
import { useMutation, useQueryClient } from '@tanstack/react-query'
import {
  postProducts,
  postProductsByProductIdVariants,
  postMediaBatch,
} from '../../client/sdk.gen'
import type { Currency } from '../../client/types.gen'
import { getProductsQueryKey } from '../../client/@tanstack/react-query.gen'
import { notifications } from '@mantine/notifications'
import { ArrowLeft, FileJson } from 'lucide-react'

export const Route = createFileRoute('/products/import')({
  component: ImportProductsPage,
})

type ImportVariant = {
  name: string
  price: {
    currency: string
    price: string
  }
  description: string
  images: string[]
  tags: string[]
}

type ImportProduct = {
  product_name: string
  product_description: string
  product_images: string[]
  variants: ImportVariant[]
}

function ImportProductsPage() {
  const navigate = useNavigate()
  const queryClient = useQueryClient()

  const form = useForm({
    initialValues: {
      json: '',
    },
    validate: {
      json: (value) => {
        try {
          const parsed = JSON.parse(value)
          if (!Array.isArray(parsed)) return 'JSON must be an array of products'
          return null
        } catch (e) {
          return 'Invalid JSON format'
        }
      },
    },
  })

  const mutation = useMutation({
    mutationFn: async (values: typeof form.values) => {
      const products: ImportProduct[] = JSON.parse(values.json)
      let successCount = 0

      // 1. Collect all URLs
      const allUrls = new Set<string>()
      for (const product of products) {
        product.product_images.forEach((url) => allUrls.add(url))
        for (const variant of product.variants) {
          variant.images.forEach((url) => allUrls.add(url))
        }
      }

      // 2. Create media in batch
      const urlToId = new Map<string, string>()
      const urlList = Array.from(allUrls)

      if (urlList.length > 0) {
        try {
          const mediaRes = await postMediaBatch({
            body: { urls: urlList },
          })
          if (mediaRes.data) {
            mediaRes.data.forEach((media, index) => {
              if (index < urlList.length) {
                urlToId.set(urlList[index], media.id)
              }
            })
          }
        } catch (e) {
          console.error('Failed to create media batch', e)
        }
      }

      for (const product of products) {
        try {
          const productMedias = product.product_images
            .map((url) => urlToId.get(url))
            .filter((id): id is string => !!id)

          const productRes = await postProducts({
            body: {
              name: product.product_name,
              description: product.product_description,
              medias: productMedias,
            },
          })

          if (!productRes.data) {
            console.error('Failed to create product', product.product_name)
            continue
          }

          const productId = productRes.data.id

          await Promise.all(
            product.variants.map((variant, index) => {
              const priceAmount = Number.parseInt(
                variant.price.price.replace(/,/g, ''),
                10,
              )

              const finalPriceAmount = priceAmount

              const variantMedias = variant.images
                .map((url) => urlToId.get(url))
                .filter((id): id is string => !!id)

              return postProductsByProductIdVariants({
                path: { product_id: productId },
                body: {
                  name: variant.name,
                  description: variant.description,
                  price: {
                    amount: Number.isNaN(finalPriceAmount)
                      ? 0
                      : finalPriceAmount,
                    currency: variant.price.currency as Currency,
                  },
                  sort_order: index,
                  medias: variantMedias,
                  tags: variant.tags,
                },
              })
            }),
          )
          successCount++
        } catch (e) {
          console.error('Error importing product', product.product_name, e)
        }
      }

      return successCount
    },
    onSuccess: (count) => {
      notifications.show({
        title: 'Success',
        message: `Successfully imported ${count} products`,
        color: 'green',
      })
      queryClient.invalidateQueries({ queryKey: getProductsQueryKey() })
      navigate({ to: '/products' })
    },
    onError: (error) => {
      notifications.show({
        title: 'Error',
        message: error.message || 'Failed to import products',
        color: 'red',
      })
    },
  })

  const handleSubmit = (values: typeof form.values) => {
    mutation.mutate(values)
  }

  return (
    <Container
      size="md"
      py="xl"
      className="[view-transition-name:import-products-page]"
    >
      <Button
        variant="subtle"
        color="gray"
        mb="lg"
        leftSection={<ArrowLeft size={16} />}
        component={Link}
        to="/products"
        viewTransition
      >
        Back to Products
      </Button>

      <Group justify="space-between" mb="xl">
        <div>
          <Title order={2} className="font-bold text-gray-900 dark:text-white">
            Import Products
          </Title>
          <Text c="dimmed" size="sm">
            Import products from JSON
          </Text>
        </div>
      </Group>

      <form onSubmit={form.onSubmit(handleSubmit)}>
        <Stack gap="xl">
          <Paper
            shadow="sm"
            radius="md"
            p="xl"
            withBorder
            className="bg-white dark:bg-gray-800"
          >
            <Group mb="md">
              <ThemeIcon size="lg" radius="md" variant="light" color="violet">
                <FileJson size={20} />
              </ThemeIcon>
              <Text fw={600} size="lg">
                JSON Data
              </Text>
            </Group>

            <Stack gap="md">
              <Textarea
                label="Paste JSON here"
                placeholder='[{"product_name": "...", ...}]'
                minRows={15}
                autosize
                maxRows={30}
                classNames={{
                  input: 'font-mono text-xs focus:border-violet-500',
                }}
                {...form.getInputProps('json')}
              />

              <Text size="sm" fw={500}>
                Example Format:
              </Text>
              <Code block>
                {`[
  {
    "product_name": "Product Name",
    "product_description": "Description",
    "product_images": [],
    "variants": [
      {
        "name": "Variant Name",
        "price": {
          "currency": "JPY",
          "price": "2,900"
        },
        "description": "",
        "images": [],
        "tags": ["tag1", "tag2"]
      }
    ]
  }
]`}
              </Code>
            </Stack>
          </Paper>

          <Group justify="flex-end" mt="md">
            <Button
              type="submit"
              loading={mutation.isPending}
              size="md"
              color="violet"
              className="bg-violet-600 hover:bg-violet-700 transition-colors duration-200"
            >
              Import Products
            </Button>
          </Group>
        </Stack>
      </form>
    </Container>
  )
}
