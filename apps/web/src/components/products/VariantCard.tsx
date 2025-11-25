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
import { Link } from '@tanstack/react-router'
import { useTag } from '../../hooks/useTags'

interface VariantCardProps {
  variant: ProductVariant
  id?: string
  onTagClick?: (tagId: string) => void
  onClick?: () => void
}

function TagBadge({
  id,
  onClick,
}: {
  id: string
  onClick?: (id: string) => void
}) {
  const { data: tag } = useTag(id)
  return (
    <Badge
      variant="light"
      color="gray"
      size="sm"
      radius="sm"
      className={
        onClick
          ? 'hover:bg-gray-200 dark:hover:bg-gray-700 transition-colors'
          : ''
      }
      style={{ cursor: onClick ? 'pointer' : 'default' }}
      onClick={(e) => {
        if (onClick) {
          e.preventDefault()
          e.stopPropagation()
          onClick(id)
        }
      }}
    >
      {tag?.name || id}
    </Badge>
  )
}

function VariantCardContent({
  variant,
  onTagClick,
}: {
  variant: ProductVariant
  onTagClick?: (tagId: string) => void
}) {
  const { addToCart, items } = useCart()

  return (
    <>
      <Card.Section className="relative">
        <Image
          src={
            variant.medias && variant.medias.length > 0
              ? getImageUrl(variant.medias[0])
              : 'https://placehold.co/600x400/f3f4f6/a78bfa?text=Variant'
          }
          h={200}
          w="100%"
          fit="contain"
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

        {variant.tags && variant.tags.length > 0 && (
          <Group gap={4} mt="xs">
            {variant.tags.map((tagId) => (
              <TagBadge key={tagId} id={tagId} onClick={onTagClick} />
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
              onClick={(e) => {
                e.preventDefault()
                e.stopPropagation()
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
    </>
  )
}

export function VariantCard({
  variant,
  id,
  onTagClick,
  onClick,
}: VariantCardProps) {
  const commonProps = {
    id,
    padding: '0' as const,
    radius: 'md' as const,
    withBorder: true,
    className:
      'cursor-pointer hover:shadow-xl transition-all duration-300 hover:-translate-y-1 overflow-hidden bg-white dark:bg-gray-800 h-full flex flex-col',
  }

  if (onClick) {
    return (
      <Card component="div" onClick={onClick} {...commonProps}>
        <VariantCardContent variant={variant} onTagClick={onTagClick} />
      </Card>
    )
  }

  return (
    <Card
      component={Link}
      to={`/products/${variant.product_id}`}
      hash={`variant-${variant.id}`}
      {...commonProps}
    >
      <VariantCardContent variant={variant} onTagClick={onTagClick} />
    </Card>
  )
}
