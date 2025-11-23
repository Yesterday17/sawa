import { createFileRoute, Link } from '@tanstack/react-router'
import { useQuery } from '@tanstack/react-query'
import {
  getProductsByProductIdOptions,
  getProductsByProductIdVariantsOptions,
} from '../../client/@tanstack/react-query.gen'
import {
  Container,
  Title,
  Text,
  Loader,
  Center,
  SimpleGrid,
  Card,
  Image,
  Stack,
  Group,
  Badge,
  Button,
  ActionIcon,
  Indicator,
} from '@mantine/core'
import { ArrowLeft, ShoppingCart } from 'lucide-react'
import { useCart } from '../../context/CartContext'
import { notifications } from '@mantine/notifications'

export const Route = createFileRoute('/products/$productId')({
  component: ProductDetailPage,
})

function ProductDetailPage() {
  const { productId } = Route.useParams()
  const { addToCart, items } = useCart()

  const {
    data: product,
    isLoading: isLoadingProduct,
    error: productError,
  } = useQuery(
    getProductsByProductIdOptions({
      path: { product_id: productId },
    }),
  )

  const {
    data: variants,
    isLoading: isLoadingVariants,
    error: variantsError,
  } = useQuery(
    getProductsByProductIdVariantsOptions({
      path: { product_id: productId },
    }),
  )

  if (isLoadingProduct || isLoadingVariants)
    return (
      <Center h={400}>
        <Loader type="dots" />
      </Center>
    )

  if (productError || variantsError)
    return (
      <Center h={400}>
        <Text c="red">Error loading product details</Text>
      </Center>
    )

  if (!product) return null

  return (
    <Container
      size="md"
      py="xl"
      className="[view-transition-name:products-page]"
    >
      <Button
        component={Link}
        to="/products"
        variant="subtle"
        leftSection={<ArrowLeft size={16} />}
        mb="md"
        viewTransition
      >
        Back to Products
      </Button>

      <Group align="start" mb="xl">
        <Image
          src="https://placehold.co/600x400/f3f4f6/a78bfa?text=Product"
          radius="md"
          h={300}
          w={400}
          fit="cover"
          className="hidden md:block"
        />
        <Stack flex={1}>
          <Title order={1}>{product.name}</Title>
          <Text size="lg" c="dimmed">
            {product.description}
          </Text>
          <Group>
            <Badge size="lg" variant="light" color="violet">
              {variants?.length || 0} Variants
            </Badge>
          </Group>
        </Stack>
      </Group>

      <Title order={3} mb="md">
        Variants
      </Title>

      <SimpleGrid cols={{ base: 1, sm: 2, md: 3 }} spacing="md">
        {variants?.map((variant) => (
          <Card key={variant.id} withBorder padding="md" radius="md">
            <Card.Section>
              <Image
                src="https://placehold.co/600x400/f3f4f6/a78bfa?text=Variant"
                height={160}
                alt={variant.name}
              />
            </Card.Section>

            <Stack mt="md" gap="xs">
              <Text fw={700} size="lg">
                {variant.name}
              </Text>
              <Text size="sm" c="dimmed" lineClamp={2}>
                {variant.description}
              </Text>
              <Group justify="space-between" mt="xs" align="flex-end">
                <div>
                  {variant.price && (
                    <Text fw={700} size="xl" c="violet">
                      {(variant.price.amount / 100).toFixed(2)}{' '}
                      {variant.price.currency}
                    </Text>
                  )}
                </div>

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
    </Container>
  )
}
