import { createFileRoute, Link } from '@tanstack/react-router'
import { useQuery } from '@tanstack/react-query'
import {
  getProductsOptions,
  getProductsVariantsOptions,
} from '../../client/@tanstack/react-query.gen'
import {
  ActionIcon,
  Badge,
  Button,
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
import { Plus, Package, Layers, ShoppingCart, Upload } from 'lucide-react'
import { useState } from 'react'
import { flushSync } from 'react-dom'
import { useCart } from '../../context/CartContext'
import { notifications } from '@mantine/notifications'
import { client } from '../../client/client.gen'
import { formatPrice } from '../../lib/utils'

export const Route = createFileRoute('/products/')({
  component: ProductsPage,
})

const getImageUrl = (mediaId: string) => {
  const baseUrl = client.getConfig().baseUrl
  return `${baseUrl}/media/${mediaId}`
}

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
        <Group>
          {viewMode === 'products' && (
            <Button
              leftSection={<Upload size={16} />}
              variant="light"
              color="violet"
              component={Link}
              to="/products/import"
            >
              Import
            </Button>
          )}
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
      </Group>

      {viewMode === 'products' ? (
        <SimpleGrid cols={{ base: 2, sm: 3, md: 4 }} spacing="md">
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
                  src={
                    product.medias && product.medias.length > 0
                      ? getImageUrl(product.medias[0])
                      : 'https://placehold.co/600x400/f3f4f6/a78bfa?text=Product'
                  }
                  h={200}
                  w="100%"
                  fit="cover"
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

                <Text size="sm" c="dimmed" lineClamp={2}>
                  {product.description ||
                    'No description available for this product.'}
                </Text>

                <Group mt="auto">
                  <Badge variant="light" color="violet" size="sm">
                    {product.medias?.length || 0} Images
                  </Badge>
                </Group>
              </Stack>
            </Card>
          ))}
        </SimpleGrid>
      ) : (
        <SimpleGrid cols={{ base: 2, sm: 3, md: 4 }} spacing="md">
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
                  src={
                    variant.medias && variant.medias.length > 0
                      ? getImageUrl(variant.medias[0])
                      : 'https://placehold.co/600x400/f3f4f6/a78bfa?text=Variant'
                  }
                  h={200}
                  w="100%"
                  fit="cover"
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

                <Text size="sm" c="dimmed" lineClamp={2}>
                  {variant.description ||
                    'No description available for this variant.'}
                </Text>

                {variant.tags && variant.tags.length > 0 && (
                  <Group gap={4} mt="xs">
                    {variant.tags.map((tag) => (
                      <Badge key={tag} variant="outline" color="gray" size="xs">
                        {tag}
                      </Badge>
                    ))}
                  </Group>
                )}

                <Group mt="auto" justify="space-between">
                  <Group gap="xs">
                    {variant.price && (
                      <Badge variant="light" color="green" size="sm">
                        {formatPrice(variant.price)}
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
