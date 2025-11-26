import { createFileRoute, Link, useLocation } from '@tanstack/react-router'
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
  ActionIcon,
  Modal,
} from '@mantine/core'
import { ArrowLeft, X } from 'lucide-react'
import { getImageUrl } from '../../lib/utils'
import { VariantCard } from '../../components/products/VariantCard'
import { VariantSwipeView } from '../../components/products/VariantSwipeView'
import { useEffect, useState, useMemo, useCallback } from 'react'
import { useTag } from '../../hooks/useTags'
import type { ProductVariant } from '../../client/types.gen'

export const Route = createFileRoute('/products/$productId')({
  component: ProductDetailPage,
})

function SelectedTagBadge({
  id,
  onRemove,
}: {
  id: string
  onRemove?: (id: string) => void
}) {
  const { data: tag } = useTag(id)
  return (
    <Badge
      variant="filled"
      color="violet"
      size="lg"
      rightSection={
        onRemove ? (
          <ActionIcon
            size="sm"
            color="white"
            variant="transparent"
            onClick={() => onRemove(id)}
          >
            <X size={14} />
          </ActionIcon>
        ) : undefined
      }
    >
      {tag?.name || id}
    </Badge>
  )
}

function ProductDetailPage() {
  const { productId } = Route.useParams()
  const { hash } = useLocation()
  const [selectedTags, setSelectedTags] = useState<string[]>([])
  const [selectedVariant, setSelectedVariant] = useState<ProductVariant | null>(
    null,
  )
  const [slideDirection, setSlideDirection] = useState<'left' | 'right' | null>(
    null,
  )

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

  const filteredVariants = useMemo(() => {
    if (!variants) return []
    if (selectedTags.length === 0) return variants
    return variants.filter((variant) =>
      selectedTags.every((tagId) => variant.tags?.includes(tagId)),
    )
  }, [variants, selectedTags])

  const handleTagClick = (tagId: string) => {
    if (!selectedTags.includes(tagId)) {
      setSelectedTags([...selectedTags, tagId])
    }
  }

  const handleRemoveTag = (tagId: string) => {
    setSelectedTags(selectedTags.filter((id) => id !== tagId))
  }

  const handleNextVariant = useCallback(() => {
    if (!selectedVariant || !filteredVariants.length) return
    setSlideDirection('right')
    const currentIndex = filteredVariants.findIndex(
      (v) => v.id === selectedVariant.id,
    )
    const nextIndex = (currentIndex + 1) % filteredVariants.length
    setSelectedVariant(filteredVariants[nextIndex])
  }, [selectedVariant, filteredVariants])

  const handlePrevVariant = useCallback(() => {
    if (!selectedVariant || !filteredVariants.length) return
    setSlideDirection('left')
    const currentIndex = filteredVariants.findIndex(
      (v) => v.id === selectedVariant.id,
    )
    const prevIndex =
      (currentIndex - 1 + filteredVariants.length) % filteredVariants.length
    setSelectedVariant(filteredVariants[prevIndex])
  }, [selectedVariant, filteredVariants])

  useEffect(() => {
    const handleKeyDown = (e: KeyboardEvent) => {
      if (!selectedVariant) return

      if (e.key === 'ArrowRight') {
        handleNextVariant()
      } else if (e.key === 'ArrowLeft') {
        handlePrevVariant()
      }
    }

    window.addEventListener('keydown', handleKeyDown)
    return () => window.removeEventListener('keydown', handleKeyDown)
  }, [selectedVariant, handleNextVariant, handlePrevVariant])

  useEffect(() => {
    if (hash && variants) {
      const id = hash.replace('#', '')
      const element = document.getElementById(id)
      if (element) {
        setTimeout(() => {
          element.scrollIntoView({ behavior: 'smooth', block: 'center' })
          element.classList.add('ring-2', 'ring-violet-500', 'ring-offset-2')
          setTimeout(() => {
            element.classList.remove(
              'ring-2',
              'ring-violet-500',
              'ring-offset-2',
            )
          }, 2000)
        }, 100)
      }
    }
  }, [hash, variants])

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
      size="lg"
      py="xl"
      className="[view-transition-name:products-page]"
    >
      <Button
        component={Link}
        to="/products"
        variant="subtle"
        color="gray"
        leftSection={<ArrowLeft size={18} />}
        mb="lg"
        viewTransition
        radius="xl"
        size="sm"
      >
        Back to Products
      </Button>

      <div className="bg-white dark:bg-zinc-900 rounded-3xl p-6 md:p-8 shadow-[0_2px_12px_rgba(0,0,0,0.04)] dark:shadow-none border border-zinc-100 dark:border-zinc-800 mb-12">
        <Group align="start" wrap="nowrap">
          <div className="bg-zinc-50/50 dark:bg-zinc-800/50 p-6 rounded-2xl hidden md:block">
            <Image
              src={
                product.medias && product.medias.length > 0
                  ? getImageUrl(product.medias[0])
                  : 'https://placehold.co/600x400/f3f4f6/a78bfa?text=Product'
              }
              radius="lg"
              h={280}
              w={380}
              fit="contain"
              className="mix-blend-multiply dark:mix-blend-normal"
            />
          </div>
          <Stack flex={1} gap="lg" py="xs">
            <div>
              <Title order={1} fw={800} className="text-3xl md:text-4xl mb-2">
                {product.name}
              </Title>
              <Text size="lg" c="dimmed" lh={1.6}>
                {product.description}
              </Text>
            </div>
            <Group mt="auto">
              <Badge
                size="xl"
                variant="light"
                color="violet"
                radius="md"
                className="normal-case font-medium"
              >
                {variants?.length || 0} Variants Available
              </Badge>
            </Group>
          </Stack>
        </Group>
      </div>

      <Title order={3} mb="lg" fw={700}>
        Available Variants
      </Title>

      {selectedTags.length > 0 && (
        <Group mb="md">
          <Text size="sm" fw={500}>
            Filtered by:
          </Text>
          {selectedTags.map((tagId) => (
            <SelectedTagBadge
              key={tagId}
              id={tagId}
              onRemove={handleRemoveTag}
            />
          ))}
          <Button
            variant="subtle"
            size="xs"
            color="red"
            onClick={() => setSelectedTags([])}
            radius="xl"
          >
            Clear all
          </Button>
        </Group>
      )}

      <SimpleGrid cols={{ base: 2, sm: 3, md: 4 }} spacing="lg">
        {filteredVariants?.map((variant) => (
          <VariantCard
            key={variant.id}
            variant={variant}
            id={`variant-${variant.id}`}
            onTagClick={handleTagClick}
            onClick={() => {
              setSlideDirection(null)
              setSelectedVariant(variant)
            }}
          />
        ))}
      </SimpleGrid>

      <Modal
        opened={!!selectedVariant}
        onClose={() => setSelectedVariant(null)}
        fullScreen
        transitionProps={{ transition: 'fade', duration: 200 }}
        padding={0}
        withCloseButton={false}
        zIndex={200}
      >
        {selectedVariant && (
          <div className="h-screen flex flex-col bg-white dark:bg-black text-black dark:text-white relative">
            <VariantSwipeView
              variant={selectedVariant}
              onNext={handleNextVariant}
              onPrev={handlePrevVariant}
              onClose={() => setSelectedVariant(null)}
              direction={slideDirection}
            />
          </div>
        )}
      </Modal>
    </Container>
  )
}
