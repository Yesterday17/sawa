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
      radius="lg"
      component={Link}
      to={`/products/${product.id}`}
      viewTransition
      className="cursor-pointer shadow-[0_2px_12px_rgba(0,0,0,0.04)] dark:shadow-none hover:shadow-[0_4px_20px_rgba(0,0,0,0.08)] dark:hover:shadow-none transition-all duration-300 hover:-translate-y-1 overflow-hidden bg-white dark:bg-zinc-900 h-full flex flex-col border border-zinc-100 dark:border-zinc-800"
    >
      <Card.Section className="relative bg-zinc-50/50 dark:bg-zinc-800/50 p-4">
        <Image
          src={
            product.medias && product.medias.length > 0
              ? getImageUrl(product.medias[0])
              : 'https://placehold.co/600x400/f3f4f6/a78bfa?text=Product'
          }
          h={180}
          w="100%"
          fit="contain"
          alt={product.name}
          className="transition-transform hover:scale-105 duration-500 mix-blend-multiply dark:mix-blend-normal"
        />
      </Card.Section>

      <Stack gap="xs" p="lg" className="flex-1">
        <Group justify="space-between" align="start" wrap="nowrap">
          <Text
            fw={600}
            size="lg"
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
          <Badge variant="light" color="violet" size="sm" radius="sm">
            {product.medias?.length || 0} Images
          </Badge>
        </Group>
      </Stack>
    </Card>
  )
}
