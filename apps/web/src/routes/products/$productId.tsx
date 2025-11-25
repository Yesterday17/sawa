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
import { client } from '../../client/client.gen'
import { formatPrice } from '../../lib/utils'

export const Route = createFileRoute('/products/$productId')({
  component: ProductDetailPage,
})

const getImageUrl = (mediaId: string) => {
  const baseUrl = client.getConfig().baseUrl
  return `${baseUrl}/media/${mediaId}`
}

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
          src={
            product.medias && product.medias.length > 0
              ? getImageUrl(product.medias[0])
              : 'https://placehold.co/600x400/f3f4f6/a78bfa?text=Product'
          }
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
    </Container>
  )
}
