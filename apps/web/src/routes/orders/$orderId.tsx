import { createFileRoute, Link } from '@tanstack/react-router'
import { useQuery, useMutation, useQueryClient } from '@tanstack/react-query'
import {
  getOrdersByOrderIdQueryKey,
  postOrdersByOrderIdItemsByItemIdMysteryBoxMutation,
  postOrdersByOrderIdFulfillMutation,
  postOrdersByOrderIdCancelMutation,
  getOrdersByOrderIdOptions,
  getProductsVariantsOptions,
} from '../../client/@tanstack/react-query.gen'
import {
  Container,
  Title,
  Text,
  Group,
  Badge,
  Card,
  SimpleGrid,
  Button,
  Modal,
  MultiSelect,
  Loader,
  Center,
  Stack,
  Paper,
  ThemeIcon,
  Divider,
  Timeline,
  Image,
} from '@mantine/core'
import { useDisclosure } from '@mantine/hooks'
import { useState } from 'react'
import type { PurchaseOrderItem, ProductVariant } from '../../client/types.gen'
import { useAuth } from '../../context/AuthContext'
import {
  ArrowLeft,
  Package,
  Calendar,
  CreditCard,
  Box,
  CheckCircle2,
  Clock,
  AlertCircle,
  Check,
  X,
} from 'lucide-react'
import { notifications } from '@mantine/notifications'
import { formatPrice, getImageUrl } from '../../lib/utils'

export const Route = createFileRoute('/orders/$orderId')({
  component: OrderDetailsPage,
})

function OrderDetailsPage() {
  const { orderId } = Route.useParams()
  const queryClient = useQueryClient()

  const fulfillOrderMutation = useMutation(postOrdersByOrderIdFulfillMutation())
  const cancelOrderMutation = useMutation(postOrdersByOrderIdCancelMutation())

  const {
    data: order,
    isLoading,
    error,
  } = useQuery(
    getOrdersByOrderIdOptions({
      path: { order_id: orderId },
    }),
  )

  const { data: allVariants } = useQuery(getProductsVariantsOptions())

  const handleFulfill = async () => {
    try {
      await fulfillOrderMutation.mutateAsync({
        path: { order_id: orderId },
      })
      notifications.show({
        title: 'Order Fulfilled',
        message: 'Your order has been successfully fulfilled.',
        color: 'green',
      })
      queryClient.invalidateQueries({
        queryKey: getOrdersByOrderIdQueryKey({
          path: { order_id: orderId },
        }),
      })
    } catch (error) {
      notifications.show({
        title: 'Error',
        message: 'Failed to fulfill order',
        color: 'red',
      })
    }
  }

  const handleCancel = async () => {
    if (!confirm('Are you sure you want to cancel this order?')) return

    try {
      await cancelOrderMutation.mutateAsync({
        path: { order_id: orderId },
        body: { reason: 'User cancelled' },
      })
      notifications.show({
        title: 'Order Cancelled',
        message: 'Your order has been cancelled.',
        color: 'blue',
      })
      queryClient.invalidateQueries({
        queryKey: getOrdersByOrderIdQueryKey({
          path: { order_id: orderId },
        }),
      })
    } catch (error) {
      notifications.show({
        title: 'Error',
        message: 'Failed to cancel order',
        color: 'red',
      })
    }
  }

  if (isLoading)
    return (
      <Center h={400}>
        <Loader type="dots" />
      </Center>
    )
  if (error)
    return (
      <Center h={400}>
        <Text c="red">Error loading order</Text>
      </Center>
    )
  if (!order)
    return (
      <Center h={400}>
        <Text>Order not found</Text>
      </Center>
    )

  return (
    <Container size="lg" py="xl">
      <Button
        variant="subtle"
        color="gray"
        mb="lg"
        component={Link}
        to="/orders"
        viewTransition
        leftSection={<ArrowLeft size={18} />}
        radius="xl"
        size="sm"
      >
        Back to Orders
      </Button>

      <Stack gap="xl">
        <div className="bg-white dark:bg-zinc-900 p-6 md:p-8 rounded-3xl border border-zinc-100 dark:border-zinc-800 shadow-[0_2px_12px_rgba(0,0,0,0.04)] dark:shadow-none">
          <Group justify="space-between" align="start">
            <div>
              <Group gap="sm" mb="xs">
                <Title
                  order={2}
                  className="font-bold text-zinc-900 dark:text-white"
                >
                  Order #{order.id.substring(0, 8)}
                </Title>
                <Badge
                  size="lg"
                  variant="light"
                  radius="md"
                  color={
                    order.status === 'fulfilled'
                      ? 'green'
                      : order.status === 'cancelled'
                        ? 'red'
                        : 'blue'
                  }
                >
                  {order.status}
                </Badge>
              </Group>
              <Text c="dimmed" size="md">
                Placed on {new Date(order.created_at).toLocaleDateString()} at{' '}
                {new Date(order.created_at).toLocaleTimeString()}
              </Text>
            </div>

            {order.status === 'incomplete' && (
              <Group>
                <Button
                  color="red"
                  variant="light"
                  radius="xl"
                  leftSection={<X size={18} />}
                  onClick={handleCancel}
                  loading={cancelOrderMutation.isPending}
                >
                  Cancel Order
                </Button>
                <Button
                  color="green"
                  radius="xl"
                  leftSection={<Check size={18} />}
                  onClick={handleFulfill}
                  loading={fulfillOrderMutation.isPending}
                  className="shadow-lg shadow-green-500/20 hover:shadow-green-500/40 transition-all hover:-translate-y-0.5"
                >
                  Fulfill Order
                </Button>
              </Group>
            )}
          </Group>
        </div>

        <SimpleGrid cols={{ base: 1, md: 3 }} spacing="lg">
          <Paper
            shadow="sm"
            radius="xl"
            p="xl"
            className="bg-white dark:bg-zinc-900 border border-zinc-100 dark:border-zinc-800 md:col-span-2 shadow-[0_2px_12px_rgba(0,0,0,0.04)] dark:shadow-none"
          >
            <Title
              order={4}
              mb="lg"
              className="flex items-center gap-3"
              fw={700}
            >
              <ThemeIcon size="lg" radius="md" variant="light" color="violet">
                <Package size={20} />
              </ThemeIcon>
              Order Items
            </Title>
            <Stack gap="lg">
              {order.items.map((item) => (
                <OrderItemCard
                  key={item.id}
                  item={item}
                  orderId={order.id}
                  allVariants={allVariants || []}
                  onUpdate={() =>
                    queryClient.invalidateQueries({
                      queryKey: getOrdersByOrderIdQueryKey({
                        path: { order_id: orderId },
                      }),
                    })
                  }
                />
              ))}
            </Stack>
          </Paper>

          <Stack gap="lg">
            <Paper
              shadow="sm"
              radius="xl"
              p="xl"
              className="bg-white dark:bg-zinc-900 border border-zinc-100 dark:border-zinc-800 shadow-[0_2px_12px_rgba(0,0,0,0.04)] dark:shadow-none"
            >
              <Title
                order={4}
                mb="lg"
                className="flex items-center gap-3"
                fw={700}
              >
                <ThemeIcon size="lg" radius="md" variant="light" color="violet">
                  <CreditCard size={20} />
                </ThemeIcon>
                Summary
              </Title>
              <Stack gap="md">
                <Group justify="space-between">
                  <Text c="dimmed">Subtotal</Text>
                  <Text fw={600}>
                    {order.total_price ? formatPrice(order.total_price) : '-'}
                  </Text>
                </Group>
                <Divider color="gray.2" />
                <Group justify="space-between">
                  <Text fw={700} size="lg">
                    Total
                  </Text>
                  <Text fw={800} size="xl" c="violet">
                    {order.total_price ? formatPrice(order.total_price) : '-'}
                  </Text>
                </Group>
              </Stack>
            </Paper>

            <Paper
              shadow="sm"
              radius="xl"
              p="xl"
              className="bg-white dark:bg-zinc-900 border border-zinc-100 dark:border-zinc-800 shadow-[0_2px_12px_rgba(0,0,0,0.04)] dark:shadow-none"
            >
              <Title
                order={4}
                mb="lg"
                className="flex items-center gap-3"
                fw={700}
              >
                <ThemeIcon size="lg" radius="md" variant="light" color="violet">
                  <Calendar size={20} />
                </ThemeIcon>
                Timeline
              </Title>
              <Timeline
                active={order.status === 'fulfilled' ? 2 : 1}
                bulletSize={28}
                lineWidth={2}
              >
                <Timeline.Item
                  bullet={<Clock size={14} />}
                  title="Order Placed"
                >
                  <Text c="dimmed" size="sm" mt={4}>
                    {new Date(order.created_at).toLocaleString()}
                  </Text>
                </Timeline.Item>
                <Timeline.Item
                  bullet={<Package size={14} />}
                  title="Processing"
                >
                  <Text c="dimmed" size="sm" mt={4}>
                    Order is being prepared
                  </Text>
                </Timeline.Item>
                <Timeline.Item
                  bullet={<CheckCircle2 size={14} />}
                  title="Fulfilled"
                  color="green"
                >
                  <Text c="dimmed" size="sm" mt={4}>
                    {order.completed_at
                      ? new Date(order.completed_at).toLocaleString()
                      : 'Pending'}
                  </Text>
                </Timeline.Item>
              </Timeline>
            </Paper>
          </Stack>
        </SimpleGrid>
      </Stack>
    </Container>
  )
}

function OrderItemCard({
  item,
  orderId,
  allVariants,
  onUpdate,
}: {
  item: PurchaseOrderItem
  orderId: string
  allVariants: ProductVariant[]
  onUpdate: () => void
}) {
  const { user } = useAuth()
  const [opened, { open, close }] = useDisclosure(false)
  const [selectedVariants, setSelectedVariants] = useState<string[]>([])
  const submitMysteryBoxMutation = useMutation(
    postOrdersByOrderIdItemsByItemIdMysteryBoxMutation(),
  )

  const variant = allVariants.find((v) => v.id === item.purchased_variant_id)
  const isMysteryBox = !!variant?.mystery_box
  const needsInput = item.status === 'awaiting_input'

  const handleSubmit = async () => {
    try {
      await submitMysteryBoxMutation.mutateAsync({
        path: { order_id: orderId, item_id: item.id },
        body: {
          owner_id: item.line_items?.[0]?.owner_id || user?.id || '',
          received_variants: selectedVariants,
        },
      })
      close()
      onUpdate()
    } catch (error) {
      console.error(error)
    }
  }

  const possibleVariants =
    variant?.mystery_box?.possible_variants.map((id: string) => {
      const v = allVariants.find((av) => av.id === id)
      return { value: id, label: v?.name || id }
    }) || []

  return (
    <Card
      padding="lg"
      radius="lg"
      className="bg-zinc-50/50 dark:bg-zinc-800/50 border border-zinc-100 dark:border-zinc-700"
    >
      <Group justify="space-between" align="start" mb="md">
        <Group align="start" wrap="nowrap">
          <div className="bg-white dark:bg-zinc-800 p-2 rounded-xl border border-zinc-100 dark:border-zinc-700">
            {variant?.medias && variant.medias.length > 0 ? (
              <Image
                src={getImageUrl(variant.medias[0])}
                w={64}
                h={64}
                radius="md"
                fit="contain"
              />
            ) : (
              <ThemeIcon
                variant="light"
                color={isMysteryBox ? 'violet' : 'gray'}
                size={64}
                radius="md"
              >
                {isMysteryBox ? <Box size={32} /> : <Package size={32} />}
              </ThemeIcon>
            )}
          </div>
          <div>
            <Text fw={600} size="lg" className="text-gray-900 dark:text-white">
              {variant?.name || item.purchased_variant_id}
            </Text>
            <Text size="sm" c="dimmed" mt={2}>
              Quantity: {item.quantity}
            </Text>
            {isMysteryBox && (
              <Badge
                size="sm"
                variant="gradient"
                gradient={{ from: 'violet', to: 'blue' }}
                mt="xs"
              >
                Mystery Box
              </Badge>
            )}
          </div>
        </Group>
        <Badge
          size="lg"
          variant={needsInput ? 'filled' : 'light'}
          radius="md"
          color={
            needsInput
              ? 'orange'
              : item.status === 'fulfilled'
                ? 'green'
                : 'gray'
          }
        >
          {item.status === 'awaiting_input' ? 'Action Required' : item.status}
        </Badge>
      </Group>

      {needsInput && isMysteryBox && (
        <>
          <Button
            mt="md"
            fullWidth
            onClick={open}
            color="violet"
            radius="xl"
            leftSection={<AlertCircle size={18} />}
            className="animate-pulse shadow-lg shadow-violet-500/20"
          >
            Fill Mystery Box Contents
          </Button>
          <Modal
            opened={opened}
            onClose={close}
            title="Fill Mystery Box"
            centered
            radius="lg"
            overlayProps={{ blur: 3 }}
          >
            <Text mb="lg" size="sm" c="dimmed">
              Select the variants that were received in this mystery box.
            </Text>
            <MultiSelect
              label="Received Variants"
              placeholder="Select variants"
              data={possibleVariants}
              value={selectedVariants}
              onChange={setSelectedVariants}
              searchable
              mb="xl"
              size="md"
              radius="md"
              classNames={{ input: 'focus:border-violet-500' }}
            />
            <Button
              fullWidth
              onClick={handleSubmit}
              loading={submitMysteryBoxMutation.isPending}
              color="violet"
              size="md"
              radius="xl"
            >
              Submit Results
            </Button>
          </Modal>
        </>
      )}
    </Card>
  )
}
