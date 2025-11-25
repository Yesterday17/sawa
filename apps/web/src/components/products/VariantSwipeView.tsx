import {
  ActionIcon,
  Image,
  Container,
  Group,
  Stack,
  Title,
  Text,
  Badge,
  Indicator,
  Button,
} from '@mantine/core'
import { ChevronLeft, ChevronRight, X, ShoppingCart } from 'lucide-react'
import { useState, useEffect } from 'react'
import type { ProductVariant } from '../../client/types.gen'
import { getImageUrl, formatPrice } from '../../lib/utils'
import { useCart } from '../../context/CartContext'
import { notifications } from '@mantine/notifications'
import { useTag } from '../../hooks/useTags'

interface VariantSwipeViewProps {
  variant: ProductVariant
  onNext: () => void
  onPrev: () => void
  onClose: () => void
  direction: 'left' | 'right' | null
}

function TagBadge({ id }: { id: string }) {
  const { data: tag } = useTag(id)
  return (
    <Badge variant="filled" color="violet" size="lg">
      {tag?.name || id}
    </Badge>
  )
}

export function VariantSwipeView({
  variant,
  onNext,
  onPrev,
  onClose,
  direction,
}: VariantSwipeViewProps) {
  const [dragOffset, setDragOffset] = useState(0)
  const [isDragging, setIsDragging] = useState(false)
  const [touchStart, setTouchStart] = useState<number | null>(null)
  const [touchEnd, setTouchEnd] = useState<number | null>(null)
  const { addToCart, items } = useCart()

  const minSwipeDistance = 50

  useEffect(() => {
    setDragOffset(0)
    setIsDragging(false)
    setTouchStart(null)
    setTouchEnd(null)
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [variant.id])

  const onTouchStart = (e: React.TouchEvent) => {
    setTouchEnd(null)
    setTouchStart(e.targetTouches[0].clientX)
    setIsDragging(true)
  }

  const onTouchMove = (e: React.TouchEvent) => {
    const currentX = e.targetTouches[0].clientX
    setTouchEnd(currentX)
    if (touchStart !== null) {
      setDragOffset(currentX - touchStart)
    }
  }

  const onTouchEnd = () => {
    setIsDragging(false)
    if (!touchStart || !touchEnd) {
      setDragOffset(0)
      return
    }
    const distance = touchStart - touchEnd
    const isLeftSwipe = distance > minSwipeDistance
    const isRightSwipe = distance < -minSwipeDistance
    if (isLeftSwipe) {
      onNext()
    } else if (isRightSwipe) {
      onPrev()
    } else {
      setDragOffset(0)
    }
  }

  return (
    <>
      <style>{`
        @keyframes slideInRight {
          from { transform: translateX(100%); opacity: 0; }
          to { transform: translateX(0); opacity: 1; }
        }
        @keyframes slideInLeft {
          from { transform: translateX(-100%); opacity: 0; }
          to { transform: translateX(0); opacity: 1; }
        }
        .slide-in-right {
          animation: slideInRight 0.3s ease-out forwards;
        }
        .slide-in-left {
          animation: slideInLeft 0.3s ease-out forwards;
        }
      `}</style>
      <ActionIcon
        variant="transparent"
        color="gray"
        size="xl"
        className="absolute top-4 left-2 z-50 hover:bg-gray-100 dark:hover:bg-white/10"
        onClick={onClose}
      >
        <X size={32} />
      </ActionIcon>

      <button
        type="button"
        className="flex-1 flex items-center justify-center relative p-4 md:p-8 overflow-hidden outline-none w-full border-none bg-transparent cursor-default appearance-none"
        onClick={(e) => {
          if ((e.target as HTMLElement).tagName !== 'IMG') {
            onClose()
          }
        }}
        onTouchStart={onTouchStart}
        onTouchMove={onTouchMove}
        onTouchEnd={onTouchEnd}
      >
        <div className="absolute size-full flex justify-between px-4 pointer-events-none">
          <ActionIcon
            variant="default"
            size="xl"
            radius="xl"
            className="absolute top-1/2 z-50 -translate-y-1/2 pointer-events-auto hidden md:flex shadow-md"
            onClick={(e) => {
              e.stopPropagation()
              onPrev()
            }}
          >
            <ChevronLeft size={24} />
          </ActionIcon>

          <ActionIcon
            variant="default"
            size="xl"
            radius="xl"
            className="absolute top-1/2 z-50 -translate-y-1/2 right-4 pointer-events-auto hidden md:flex shadow-md"
            onClick={(e) => {
              e.stopPropagation()
              onNext()
            }}
          >
            <ChevronRight size={24} />
          </ActionIcon>
        </div>
        <div
          key={variant.id}
          className={`size-full flex items-center justify-center ${
            direction === 'right'
              ? 'slide-in-right'
              : direction === 'left'
                ? 'slide-in-left'
                : ''
          }`}
        >
          <div
            className="size-full flex items-center justify-center"
            style={{
              transform: isDragging
                ? `translateX(${dragOffset}px)`
                : 'translateX(0)',
              transition: isDragging ? 'none' : 'transform 0.2s ease-out',
            }}
          >
            <Image
              src={
                variant.medias && variant.medias.length > 0
                  ? getImageUrl(variant.medias[0])
                  : 'https://placehold.co/600x400/f3f4f6/a78bfa?text=Variant'
              }
              fit="contain"
              h="auto"
              w="auto"
              className="max-h-full max-w-full object-contain"
            />
          </div>
        </div>
      </button>

      <div className="bg-white/90 dark:bg-gray-900/90 backdrop-blur-sm p-6 md:p-8 border-t border-gray-200 dark:border-gray-800 w-full z-50">
        <Container size="lg" px={0}>
          <div className="flex flex-col md:flex-row justify-between items-start gap-6 md:gap-10">
            <Stack gap="sm" className="flex-1 w-full">
              <Title
                order={2}
                className="text-2xl md:text-3xl text-gray-900 dark:text-white"
              >
                {variant.name}
              </Title>
              <Text c="dimmed" size="md" lineClamp={3}>
                {variant.description ||
                  'No description available for this variant.'}
              </Text>
              {variant.tags && variant.tags.length > 0 && (
                <Group gap={6}>
                  {variant.tags.map((tagId) => (
                    <TagBadge key={tagId} id={tagId} />
                  ))}
                </Group>
              )}
            </Stack>
            <div className="flex flex-row md:flex-col items-center md:items-end justify-between w-full md:w-auto gap-6 mt-4 md:mt-0 border-t border-gray-200 dark:border-gray-800 md:border-t-0 pt-6 md:pt-0">
              {variant.price && (
                <Text
                  size="2xl"
                  fw={700}
                  c="green.6"
                  className="dark:text-green-400"
                >
                  {formatPrice(variant.price)}
                </Text>
              )}
              <Indicator
                inline
                label={
                  items.find((item) => item.variant.id === variant.id)?.quantity
                }
                size={18}
                disabled={!items.find((item) => item.variant.id === variant.id)}
                className="flex-1 md:flex-none"
              >
                <Button
                  leftSection={<ShoppingCart size={24} />}
                  color="violet"
                  size="lg"
                  fullWidth
                  className="md:w-auto"
                  onClick={() => {
                    addToCart(variant)
                    notifications.show({
                      title: 'Added to cart',
                      message: `${variant.name} has been added to your cart`,
                      color: 'green',
                    })
                  }}
                >
                  Add to Cart
                </Button>
              </Indicator>
            </div>
          </div>
        </Container>
      </div>
    </>
  )
}
