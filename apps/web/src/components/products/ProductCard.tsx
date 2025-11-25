import { Link } from '@tanstack/react-router'
import { Card, Image, Stack, Group, Text, Badge } from '@mantine/core'
import type { Product } from '../../client/types.gen'
import { getImageUrl } from '../../lib/utils'

interface ProductCardProps {
  product: Product
}

export function ProductCard({ product }: ProductCardProps) {
  return (
    <Card
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
          {product.description || 'No description available for this product.'}
        </Text>

        <Group mt="auto">
          <Badge variant="light" color="violet" size="sm">
            {product.medias?.length || 0} Images
          </Badge>
        </Group>
      </Stack>
    </Card>
  )
}
