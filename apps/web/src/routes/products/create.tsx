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
        leftSection={<ArrowLeft size={18} />}
        component={Link}
        to="/products"
        viewTransition
        radius="xl"
        size="sm"
      >
        Back to Products
      </Button>

      <Group justify="space-between" mb="xl">
        <div>
          <Title order={2} className="font-bold text-gray-900 dark:text-white">
            Create Product
          </Title>
          <Text c="dimmed" size="lg">
            Add a new item to your catalog
          </Text>
        </div>
      </Group>

      <form onSubmit={form.onSubmit(handleSubmit)}>
        <Stack gap="xl">
          <Paper
            shadow="sm"
            radius="xl"
            p="xl"
            className="bg-white dark:bg-zinc-900 border border-gray-100 dark:border-zinc-800"
          >
            <Group mb="lg">
              <ThemeIcon size="xl" radius="lg" variant="light" color="violet">
                <Package size={24} />
              </ThemeIcon>
              <Text fw={700} size="xl">
                Basic Information
              </Text>
            </Group>

            <Stack gap="lg">
              <TextInput
                label="Product Name"
                placeholder="e.g. Premium Wireless Headphones"
                required
                size="md"
                radius="md"
                classNames={{ input: 'focus:border-violet-500' }}
                {...form.getInputProps('name')}
              />

              <Textarea
                label="Description"
                placeholder="Describe your product..."
                minRows={4}
                size="md"
                radius="md"
                classNames={{ input: 'focus:border-violet-500' }}
                {...form.getInputProps('description')}
              />
            </Stack>
          </Paper>

          <Paper
            shadow="sm"
            radius="xl"
            p="xl"
            className="bg-white dark:bg-zinc-900 border border-gray-100 dark:border-zinc-800"
          >
            <Group justify="space-between" mb="lg">
              <Group>
                <ThemeIcon size="xl" radius="lg" variant="light" color="violet">
                  <Layers size={24} />
                </ThemeIcon>
                <Text fw={700} size="xl">
                  Variants
                </Text>
              </Group>
              <Button
                variant="light"
                color="violet"
                size="sm"
                radius="xl"
                leftSection={<Plus size={16} />}
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
                  padding="lg"
                  radius="lg"
                  className="bg-gray-50 dark:bg-zinc-800/50 border border-gray-200 dark:border-zinc-700"
                >
                  <Group justify="space-between" mb="md">
                    <Text fw={600} size="sm" c="violet" tt="uppercase">
                      Variant #{index + 1}
                    </Text>
                    {form.values.variants.length > 1 && (
                      <ActionIcon
                        color="red"
                        variant="subtle"
                        size="md"
                        radius="xl"
                        onClick={() => form.removeListItem('variants', index)}
                        className="hover:bg-red-50 dark:hover:bg-red-900/20"
                      >
                        <Trash size={18} />
                      </ActionIcon>
                    )}
                  </Group>

                  <Stack gap="md">
                    <TextInput
                      label="Variant Name"
                      placeholder="e.g. Red, Large"
                      required
                      size="md"
                      radius="md"
                      classNames={{ input: 'focus:border-violet-500' }}
                      {...form.getInputProps(`variants.${index}.name`)}
                    />

                    <Group grow align="flex-start">
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
                        size="md"
                        radius="md"
                        leftSection={
                          <Text size="sm" c="dimmed">
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
                        size="md"
                        radius="md"
                        classNames={{ input: 'focus:border-violet-500' }}
                        {...form.getInputProps(`variants.${index}.currency`)}
                      />
                    </Group>

                    <Textarea
                      label="Description"
                      placeholder="Variant specific description"
                      minRows={2}
                      size="md"
                      radius="md"
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
              size="lg"
              radius="xl"
              color="violet"
              className="bg-violet-600 hover:bg-violet-700 transition-all duration-200 shadow-lg shadow-violet-500/20 hover:shadow-violet-500/40 hover:-translate-y-0.5"
            >
              Create Product
            </Button>
          </Group>
        </Stack>
      </form>
    </Container>
  )
}
