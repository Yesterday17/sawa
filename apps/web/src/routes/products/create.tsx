import { createFileRoute, Link, useNavigate } from '@tanstack/react-router'
import { useForm } from '@mantine/form'
import {
  TextInput,
  Textarea,
  Button,
  Container,
  Title,
  Stack,
  Group,
  NumberInput,
  Select,
  ActionIcon,
  Card,
  Text,
  Paper,
  ThemeIcon,
} from '@mantine/core'
import { useMutation, useQueryClient } from '@tanstack/react-query'
import {
  postProducts,
  postProductsByProductIdVariants,
} from '../../client/sdk.gen'
import type { Currency } from '../../client/types.gen'
import { getProductsQueryKey } from '../../client/@tanstack/react-query.gen'
import { notifications } from '@mantine/notifications'
import { Plus, Trash, Package, Layers, ArrowLeft } from 'lucide-react'

export const Route = createFileRoute('/products/create')({
  component: CreateProductPage,
})

const getCurrencySymbol = (currency: string) => {
  switch (currency) {
    case 'USD':
      return '$'
    case 'EUR':
      return '€'
    case 'JPY':
    case 'CNY':
      return '¥'
    default:
      return '$'
  }
}

function CreateProductPage() {
  const navigate = useNavigate()
  const queryClient = useQueryClient()

  const form = useForm({
    initialValues: {
      name: '',
      description: '',
      medias: [] as string[],
      variants: [
        {
          name: 'Default',
          description: '',
          price: 0,
          currency: 'JPY',
        },
      ],
    },
    validate: {
      name: (value) =>
        value.length < 2 ? 'Name must be at least 2 characters' : null,
      variants: {
        name: (value) => (value.length < 1 ? 'Variant name is required' : null),
        price: (value) => (value < 0 ? 'Price cannot be negative' : null),
      },
    },
  })

  const mutation = useMutation({
    mutationFn: async (values: typeof form.values) => {
      const productRes = await postProducts({
        body: {
          name: values.name,
          description: values.description,
          medias: values.medias,
        },
      })

      if (!productRes.data) {
        throw new Error('Failed to create product')
      }

      const productId = productRes.data.id

      await Promise.all(
        values.variants.map((variant, index) =>
          postProductsByProductIdVariants({
            path: { product_id: productId },
            body: {
              name: variant.name,
              description: variant.description,
              price: {
                amount: variant.price,
                currency: variant.currency as Currency,
              },
              sort_order: index,
              medias: [],
              tags: [],
            },
          }),
        ),
      )

      return productRes
    },
    onSuccess: () => {
      notifications.show({
        title: 'Success',
        message: 'Product created successfully',
        color: 'green',
      })
      queryClient.invalidateQueries({ queryKey: getProductsQueryKey() })
      navigate({ to: '/products' })
    },
    onError: (error) => {
      notifications.show({
        title: 'Error',
        message: error.message || 'Failed to create product',
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
      className="[view-transition-name:create-product-page]"
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
            Create Product
          </Title>
          <Text c="dimmed" size="sm">
            Add a new item to your catalog
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
                <Package size={20} />
              </ThemeIcon>
              <Text fw={600} size="lg">
                Basic Information
              </Text>
            </Group>

            <Stack gap="md">
              <TextInput
                label="Product Name"
                placeholder="e.g. Premium Wireless Headphones"
                required
                classNames={{ input: 'focus:border-violet-500' }}
                {...form.getInputProps('name')}
              />

              <Textarea
                label="Description"
                placeholder="Describe your product..."
                minRows={4}
                classNames={{ input: 'focus:border-violet-500' }}
                {...form.getInputProps('description')}
              />
            </Stack>
          </Paper>

          <Paper
            shadow="sm"
            radius="md"
            p="xl"
            withBorder
            className="bg-white dark:bg-gray-800"
          >
            <Group justify="space-between" mb="md">
              <Group>
                <ThemeIcon size="lg" radius="md" variant="light" color="violet">
                  <Layers size={20} />
                </ThemeIcon>
                <Text fw={600} size="lg">
                  Variants
                </Text>
              </Group>
              <Button
                variant="light"
                color="violet"
                size="xs"
                leftSection={<Plus size={14} />}
                onClick={() =>
                  form.insertListItem('variants', {
                    name: '',
                    description: '',
                    price: 0,
                    currency: 'JPY',
                  })
                }
              >
                Add Variant
              </Button>
            </Group>

            <Stack gap="lg">
              {form.values.variants.map((_, index) => (
                <Card
                  key={form.key(`variants.${index}`)}
                  withBorder
                  padding="md"
                  radius="md"
                  className="bg-gray-50 dark:bg-gray-900/50 border-gray-200 dark:border-gray-700"
                >
                  <Group justify="space-between" mb="sm">
                    <Text fw={600} size="sm" c="violet">
                      Variant #{index + 1}
                    </Text>
                    {form.values.variants.length > 1 && (
                      <ActionIcon
                        color="red"
                        variant="subtle"
                        size="sm"
                        onClick={() => form.removeListItem('variants', index)}
                      >
                        <Trash size={16} />
                      </ActionIcon>
                    )}
                  </Group>

                  <Stack gap="sm">
                    <TextInput
                      label="Variant Name"
                      placeholder="e.g. Red, Large"
                      required
                      size="sm"
                      classNames={{ input: 'focus:border-violet-500' }}
                      {...form.getInputProps(`variants.${index}.name`)}
                    />

                    <Group grow>
                      <NumberInput
                        label="Price"
                        placeholder="0.00"
                        min={0}
                        decimalScale={
                          form.values.variants[index].currency === 'JPY' ? 0 : 2
                        }
                        fixedDecimalScale={
                          form.values.variants[index].currency !== 'JPY'
                        }
                        size="sm"
                        leftSection={
                          <Text size="xs" c="dimmed">
                            {getCurrencySymbol(
                              form.values.variants[index].currency,
                            )}
                          </Text>
                        }
                        classNames={{ input: 'focus:border-violet-500' }}
                        {...form.getInputProps(`variants.${index}.price`)}
                      />
                      <Select
                        label="Currency"
                        data={['USD', 'CNY', 'EUR', 'JPY']}
                        size="sm"
                        classNames={{ input: 'focus:border-violet-500' }}
                        {...form.getInputProps(`variants.${index}.currency`)}
                      />
                    </Group>

                    <Textarea
                      label="Description"
                      placeholder="Variant specific description"
                      minRows={2}
                      size="sm"
                      classNames={{ input: 'focus:border-violet-500' }}
                      {...form.getInputProps(`variants.${index}.description`)}
                    />
                  </Stack>
                </Card>
              ))}
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
              Create Product
            </Button>
          </Group>
        </Stack>
      </form>
    </Container>
  )
}
