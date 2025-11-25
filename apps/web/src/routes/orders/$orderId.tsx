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
import { client } from '../../client/client.gen'
import { formatPrice } from '../../lib/utils'

export const Route = createFileRoute('/orders/$orderId')({
  component: OrderDetailsPage,
})

const getImageUrl = (mediaId: string) => {
  const baseUrl = client.getConfig().baseUrl
  return `${baseUrl}/media/${mediaId}`
}

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
    <Container size="xl" py="xl">
      <Button
        variant="subtle"
        color="gray"
        mb="lg"
        component={Link}
        to="/orders"
        viewTransition
        leftSection={<ArrowLeft size={16} />}
      >
        Back to Orders
      </Button>

      <Stack gap="xl">
        <Group justify="space-between" align="start">
          <div>
            <Group gap="sm" mb="xs">
              <Title
                order={2}
                className="font-bold text-gray-900 dark:text-white"
              >
                Order #{order.id.substring(0, 8)}
              </Title>
              <Badge
                size="lg"
                variant="light"
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
            <Text c="dimmed" size="sm">
              Placed on {new Date(order.created_at).toLocaleDateString()} at{' '}
              {new Date(order.created_at).toLocaleTimeString()}
            </Text>
          </div>

          {order.status === 'incomplete' && (
            <Group>
              <Button
                color="red"
                variant="light"
                leftSection={<X size={16} />}
                onClick={handleCancel}
                loading={cancelOrderMutation.isPending}
              >
                Cancel Order
              </Button>
              <Button
                color="green"
                leftSection={<Check size={16} />}
                onClick={handleFulfill}
                loading={fulfillOrderMutation.isPending}
              >
                Fulfill Order
              </Button>
            </Group>
          )}
        </Group>

        <SimpleGrid cols={{ base: 1, md: 3 }} spacing="lg">
          <Paper
            shadow="sm"
            radius="md"
            p="lg"
            withBorder
            className="bg-white dark:bg-gray-800 md:col-span-2"
          >
            <Title order={4} mb="md" className="flex items-center gap-2">
              <Package size={20} className="text-violet-500" />
              Order Items
            </Title>
            <Stack gap="md">
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
              radius="md"
              p="lg"
              withBorder
              className="bg-white dark:bg-gray-800"
            >
              <Title order={4} mb="md" className="flex items-center gap-2">
                <CreditCard size={20} className="text-violet-500" />
                Summary
              </Title>
              <Stack gap="xs">
                <Group justify="space-between">
                  <Text c="dimmed">Subtotal</Text>
                  <Text fw={500}>
                    {order.total_price ? formatPrice(order.total_price) : '-'}
                  </Text>
                </Group>
                <Divider my="xs" />
                <Group justify="space-between">
                  <Text fw={700} size="lg">
                    Total
                  </Text>
                  <Text fw={700} size="lg" c="violet">
                    {order.total_price ? formatPrice(order.total_price) : '-'}
                  </Text>
                </Group>
              </Stack>
            </Paper>

            <Paper
              shadow="sm"
              radius="md"
              p="lg"
              withBorder
              className="bg-white dark:bg-gray-800"
            >
              <Title order={4} mb="md" className="flex items-center gap-2">
                <Calendar size={20} className="text-violet-500" />
                Timeline
              </Title>
              <Timeline
                active={order.status === 'fulfilled' ? 2 : 1}
                bulletSize={24}
                lineWidth={2}
              >
                <Timeline.Item
                  bullet={<Clock size={12} />}
                  title="Order Placed"
                >
                  <Text c="dimmed" size="xs" mt={4}>
                    {new Date(order.created_at).toLocaleString()}
                  </Text>
                </Timeline.Item>
                <Timeline.Item
                  bullet={<Package size={12} />}
                  title="Processing"
                >
                  <Text c="dimmed" size="xs" mt={4}>
                    Order is being prepared
                  </Text>
                </Timeline.Item>
                <Timeline.Item
                  bullet={<CheckCircle2 size={12} />}
                  title="Fulfilled"
                  color="green"
                >
                  <Text c="dimmed" size="xs" mt={4}>
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
      withBorder
      padding="md"
      radius="md"
      className="bg-gray-50 dark:bg-gray-900/50"
    >
      <Group justify="space-between" align="start" mb="xs">
        <Group>
          {variant?.medias && variant.medias.length > 0 ? (
            <Image
              src={getImageUrl(variant.medias[0])}
              w={60}
              h={60}
              radius="md"
            />
          ) : (
            <ThemeIcon
              variant="light"
              color={isMysteryBox ? 'violet' : 'gray'}
              size="lg"
              radius="md"
            >
              {isMysteryBox ? <Box size={20} /> : <Package size={20} />}
            </ThemeIcon>
          )}
          <div>
            <Text fw={600}>{variant?.name || item.purchased_variant_id}</Text>
            <Text size="xs" c="dimmed">
              Quantity: {item.quantity}
            </Text>
          </div>
        </Group>
        <Badge
          variant={needsInput ? 'filled' : 'light'}
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

      {isMysteryBox && (
        <Group gap="xs" mt="xs">
          <Badge size="xs" variant="dot" color="violet">
            Mystery Box
          </Badge>
        </Group>
      )}

      {needsInput && isMysteryBox && (
        <>
          <Button
            mt="md"
            fullWidth
            onClick={open}
            color="violet"
            leftSection={<AlertCircle size={16} />}
            className="animate-pulse"
          >
            Fill Mystery Box Contents
          </Button>
          <Modal
            opened={opened}
            onClose={close}
            title="Fill Mystery Box"
            centered
            radius="md"
          >
            <Text mb="md" size="sm" c="dimmed">
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
            />
            <Button
              fullWidth
              onClick={handleSubmit}
              loading={submitMysteryBoxMutation.isPending}
              color="violet"
            >
              Submit Results
            </Button>
          </Modal>
        </>
      )}
    </Card>
  )
}
