import { createFileRoute, Link } from '@tanstack/react-router'
import { useQuery } from '@tanstack/react-query'
import {
  getProductsOptions,
  getProductsVariantsOptions,
} from '../../client/@tanstack/react-query.gen'
import {
  ActionIcon,
  Badge,
  Card,
  Center,
  Container,
  Group,
  Image,
  Indicator,
  Loader,
  SegmentedControl,
  SimpleGrid,
  Stack,
  Text,
  ThemeIcon,
  Title,
} from '@mantine/core'
import { Plus, Package, Layers, ShoppingCart } from 'lucide-react'
import { useState } from 'react'
import { flushSync } from 'react-dom'
import { useCart } from '../../context/CartContext'
import { notifications } from '@mantine/notifications'

export const Route = createFileRoute('/products/')({
  component: ProductsPage,
})

function ProductsPage() {
  const [viewMode, setViewMode] = useState<'products' | 'variants'>('products')
  const { addToCart, items } = useCart()

  const {
    data: products,
    isLoading: isLoadingProducts,
    error: productsError,
  } = useQuery({
    ...getProductsOptions(),
    enabled: viewMode === 'products',
  })

  const {
    data: variants,
    isLoading: isLoadingVariants,
    error: variantsError,
  } = useQuery({
    ...getProductsVariantsOptions(),
    enabled: viewMode === 'variants',
  })

  const isLoading =
    viewMode === 'products' ? isLoadingProducts : isLoadingVariants
  const error = viewMode === 'products' ? productsError : variantsError

  if (isLoading)
    return (
      <Center h={400}>
        <Loader type="dots" />
      </Center>
    )
  if (error)
    return (
      <Center h={400}>
        <Text c="red">Error loading data</Text>
      </Center>
    )

  return (
    <Container size="md" py="xl">
      <Group
        justify="space-between"
        mb={40}
        className="[view-transition-name:products-header]"
      >
        <div>
          <Title order={2} className="font-bold text-gray-900 dark:text-white">
            {viewMode === 'products' ? 'Products' : 'Variants'}
          </Title>
          <Text c="dimmed" size="sm">
            Manage your{' '}
            {viewMode === 'products' ? 'product catalog' : 'product variants'}
          </Text>
        </div>
        <SegmentedControl
          value={viewMode}
          onChange={(value) => {
            const nextValue = value as 'products' | 'variants'
            if (!document.startViewTransition) {
              setViewMode(nextValue)
              return
            }
            document.startViewTransition(() => {
              flushSync(() => {
                setViewMode(nextValue)
              })
            })
          }}
          data={[
            {
              label: (
                <Center style={{ gap: 10 }}>
                  <Package size={16} />
                  <span>Products</span>
                </Center>
              ),
              value: 'products',
            },
            {
              label: (
                <Center style={{ gap: 10 }}>
                  <Layers size={16} />
                  <span>Variants</span>
                </Center>
              ),
              value: 'variants',
            },
          ]}
        />
      </Group>

      {viewMode === 'products' ? (
        <SimpleGrid cols={{ base: 2, sm: 3, md: 4, lg: 4 }} spacing="md">
          <Card
            component={Link}
            to="/products/create"
            viewTransition
            padding="md"
            radius="md"
            className="flex justify-center items-center cursor-pointer border-2 border-dashed border-gray-300 dark:border-gray-700 hover:border-violet-500 dark:hover:border-violet-500 hover:bg-violet-50 dark:hover:bg-violet-900/20 transition-all duration-300 group min-h-[220px] h-full"
            withBorder={false}
          >
            <Stack align="center" gap="md">
              <ThemeIcon
                size={60}
                radius="xl"
                variant="light"
                color="violet"
                className="group-hover:scale-110 transition-transform"
              >
                <Plus size={32} />
              </ThemeIcon>
              <Stack gap={0} align="center">
                <Text
                  fw={600}
                  size="lg"
                  className="group-hover:text-violet-600 dark:group-hover:text-violet-400 transition-colors"
                >
                  Create Product
                </Text>
                <Text c="dimmed" size="sm" ta="center">
                  Add a new item to your catalog
                </Text>
              </Stack>
            </Stack>
          </Card>

          {products?.map((product) => (
            <Card
              key={product.id}
              padding="0"
              radius="md"
              withBorder
              component={Link}
              to={`/products/${product.id}`}
              viewTransition
              className="cursor-pointer hover:shadow-xl transition-all duration-300 hover:-translate-y-1 overflow-hidden bg-white dark:bg-gray-800 h-full flex flex-col"
            >
              <Card.Section className="relative">
                <Image
                  src="https://placehold.co/600x400/f3f4f6/a78bfa?text=Product"
                  height={140}
                  alt={product.name}
                  className="transition-transform hover:scale-105 duration-500"
                />
              </Card.Section>

              <Stack gap="xs" p="md" className="flex-1">
                <Group justify="space-between" align="start" wrap="nowrap">
                  <Text
                    fw={700}
                    size="md"
                    lineClamp={1}
                    className="text-gray-900 dark:text-white"
                  >
                    {product.name}
                  </Text>
                </Group>

                <Text size="sm" c="dimmed" lineClamp={2} className="flex-1">
                  {product.description ||
                    'No description available for this product.'}
                </Text>

                <Group mt="md">
                  <Badge variant="light" color="violet" size="sm">
                    {product.medias?.length || 0} Images
                  </Badge>
                </Group>
              </Stack>
            </Card>
          ))}
        </SimpleGrid>
      ) : (
        <SimpleGrid cols={{ base: 2, sm: 3, md: 4, lg: 4 }} spacing="md">
          {variants?.map((variant) => (
            <Card
              key={variant.id}
              padding="0"
              radius="md"
              withBorder
              className="cursor-pointer hover:shadow-xl transition-all duration-300 hover:-translate-y-1 overflow-hidden bg-white dark:bg-gray-800 h-full flex flex-col"
            >
              <Card.Section className="relative">
                <Image
                  src="https://placehold.co/600x400/f3f4f6/a78bfa?text=Variant"
                  height={140}
                  alt={variant.name}
                  className="transition-transform hover:scale-105 duration-500"
                />
              </Card.Section>

              <Stack gap="xs" p="md" className="flex-1">
                <Group justify="space-between" align="start" wrap="nowrap">
                  <Text
                    fw={700}
                    size="md"
                    lineClamp={1}
                    className="text-gray-900 dark:text-white"
                  >
                    {variant.name}
                  </Text>
                </Group>

                <Text size="sm" c="dimmed" lineClamp={2} className="flex-1">
                  {variant.description ||
                    'No description available for this variant.'}
                </Text>

                <Group mt="md" justify="space-between">
                  <Group gap="xs">
                    {variant.price && (
                      <Badge variant="light" color="green" size="sm">
                        {variant.price.amount / 100} {variant.price.currency}
                      </Badge>
                    )}
                  </Group>

                  <Indicator
                    inline
                    label={
                      items.find((item) => item.variant.id === variant.id)
                        ?.quantity
                    }
                    size={16}
                    disabled={
                      !items.find((item) => item.variant.id === variant.id)
                    }
                  >
                    <ActionIcon
                      variant="light"
                      color="violet"
                      size="lg"
                      onClick={() => {
                        addToCart(variant)
                        notifications.show({
                          title: 'Added to cart',
                          message: `${variant.name} has been added to your cart`,
                          color: 'green',
                        })
                      }}
                    >
                      <ShoppingCart size={18} />
                    </ActionIcon>
                  </Indicator>
                </Group>
              </Stack>
            </Card>
          ))}
        </SimpleGrid>
      )}
    </Container>
  )
}
