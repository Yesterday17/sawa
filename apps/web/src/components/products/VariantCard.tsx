import {
  Card,
  Image,
  Stack,
  Group,
  Text,
  Badge,
  Indicator,
  ActionIcon,
} from '@mantine/core'
import { ShoppingCart } from 'lucide-react'
import { notifications } from '@mantine/notifications'
import type { ProductVariant } from '../../client/types.gen'
import { getImageUrl, formatPrice } from '../../lib/utils'
import { useCart } from '../../context/CartContext'

interface VariantCardProps {
  variant: ProductVariant
}

export function VariantCard({ variant }: VariantCardProps) {
  const { addToCart, items } = useCart()

  return (
    <Card
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
          {variant.description || 'No description available for this variant.'}
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
              items.find((item) => item.variant.id === variant.id)?.quantity
            }
            size={16}
            disabled={!items.find((item) => item.variant.id === variant.id)}
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
  )
}
