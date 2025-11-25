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
  Image,
  Stack,
  Group,
  Badge,
  Button,
} from '@mantine/core'
import { ArrowLeft } from 'lucide-react'
import { getImageUrl } from '../../lib/utils'
import { VariantCard } from '../../components/products/VariantCard'

export const Route = createFileRoute('/products/$productId')({
  component: ProductDetailPage,
})

function ProductDetailPage() {
  const { productId } = Route.useParams()

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
          <VariantCard key={variant.id} variant={variant} />
        ))}
      </SimpleGrid>
    </Container>
  )
}
