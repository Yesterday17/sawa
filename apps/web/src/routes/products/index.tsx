import { createFileRoute, Link } from '@tanstack/react-router'
import { useQuery } from '@tanstack/react-query'
import {
  getProductsOptions,
  getProductsVariantsOptions,
} from '../../client/@tanstack/react-query.gen'
import {
  Button,
  Card,
  Center,
  Container,
  Group,
  Loader,
  SegmentedControl,
  SimpleGrid,
  Stack,
  Text,
  ThemeIcon,
  Title,
} from '@mantine/core'
import { Plus, Package, Layers, Upload } from 'lucide-react'
import { useState } from 'react'
import { flushSync } from 'react-dom'
import { ProductCard } from '../../components/products/ProductCard'
import { VariantCard } from '../../components/products/VariantCard'

export const Route = createFileRoute('/products/')({
  component: ProductsPage,
})

function ProductsPage() {
  const [viewMode, setViewMode] = useState<'products' | 'variants'>('products')

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
            <ProductCard key={product.id} product={product} />
          ))}
        </SimpleGrid>
      ) : (
        <SimpleGrid cols={{ base: 2, sm: 3, md: 4 }} spacing="md">
          {variants?.map((variant) => (
            <VariantCard key={variant.id} variant={variant} />
          ))}
        </SimpleGrid>
      )}
    </Container>
  )
}
