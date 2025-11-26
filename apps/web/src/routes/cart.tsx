import { createFileRoute, Link, useNavigate } from '@tanstack/react-router'
import {
  Container,
  Title,
  Text,
  Button,
  Group,
  Stack,
  Image,
  ActionIcon,
  NumberInput,
  Divider,
  Modal,
  LoadingOverlay,
} from '@mantine/core'
import { useDisclosure } from '@mantine/hooks'
import { Trash, ArrowLeft, ShoppingCart } from 'lucide-react'
import { useCart } from '../context/CartContext'
import { useAuth } from '../context/AuthContext'
import { useState } from 'react'
import { notifications } from '@mantine/notifications'
import {
  getOrders,
  postOrders,
  postOrdersByOrderIdItems,
} from '../client/sdk.gen'
import type { PurchaseOrder } from '../client/types.gen'
import { formatPrice, getImageUrl } from '../lib/utils'

export const Route = createFileRoute('/cart')({
  component: CartPage,
})

function CartPage() {
  const { items, removeFromCart, updateQuantity, clearCart, totalItems } =
    useCart()
  const { user } = useAuth()
  const navigate = useNavigate()

  const [pendingOrder, setPendingOrder] = useState<PurchaseOrder | null>(null)
  const [modalOpened, { open: openModal, close: closeModal }] =
    useDisclosure(false)
  const [loading, setLoading] = useState(false)

  const totalPrice = items.reduce((acc, item) => {
    if (item.variant.price) {
      return acc + item.variant.price.amount * item.quantity
    }
    return acc
  }, 0)

  const currency = items.find((item) => item.variant.price)?.variant.price
    ?.currency

  const handleCheckout = async () => {
    if (items.length === 0) {
      notifications.show({
        message: 'Cart is empty',
        color: 'red',
      })
      return
    }

    setLoading(true)
    try {
      const { data: orders } = await getOrders({
        query: { role: 'creator', status: 'incomplete' },
      })
      // Find the most recent incomplete order
      const pending = orders?.find((o) => o.status === 'incomplete')

      if (pending) {
        setPendingOrder(pending)
        openModal()
        setLoading(false)
      } else {
        await createNewOrder()
      }
    } catch (error) {
      console.error(error)
      notifications.show({
        title: 'Error',
        message: 'Failed to check existing orders',
        color: 'red',
      })
      setLoading(false)
    }
  }

  const createNewOrder = async () => {
    setLoading(true)
    try {
      await postOrders({
        body: {
          items: items.map((item) => ({
            variant_id: item.variant.id,
            quantity: item.quantity,
            owner_id: user?.id,
          })),
        },
      })

      clearCart()

      notifications.show({
        title: 'Success',
        message: 'Order created successfully',
        color: 'green',
      })
      navigate({ to: '/orders' })
    } catch (error) {
      console.error(error)
      notifications.show({
        title: 'Error',
        message: 'Failed to create order',
        color: 'red',
      })
    } finally {
      setLoading(false)
      closeModal()
    }
  }

  const addToPendingOrder = async () => {
    if (!pendingOrder || !user) return

    setLoading(true)
    try {
      for (const item of items) {
        await postOrdersByOrderIdItems({
          path: { order_id: pendingOrder.id },
          body: {
            variant_id: item.variant.id,
            quantity: item.quantity,
            owner_id: user.id,
          },
        })
      }

      clearCart()

      notifications.show({
        title: 'Success',
        message: 'Items added to existing order',
        color: 'green',
      })
      navigate({ to: '/orders' })
    } catch (error) {
      console.error(error)
      notifications.show({
        title: 'Error',
        message: 'Failed to add items to order',
        color: 'red',
      })
    } finally {
      setLoading(false)
      closeModal()
    }
  }

  if (items.length === 0) {
    return (
      <Container size="md" py="xl">
        <Stack align="center" gap="xl" py={100}>
          <div className="bg-gray-50 dark:bg-gray-800 p-8 rounded-full">
            <ShoppingCart size={64} className="text-gray-400" />
          </div>
          <Title order={2} fw={600}>
            Your cart is empty
          </Title>
          <Text c="dimmed" ta="center" maw={400}>
            Looks like you haven't added anything to your cart yet. Explore our
            products and find something you love!
          </Text>
          <Button
            component={Link}
            to="/products"
            size="md"
            radius="xl"
            color="violet"
            leftSection={<ArrowLeft size={18} />}
          >
            Start Shopping
          </Button>
        </Stack>
      </Container>
    )
  }

  return (
    <Container size="lg" py="xl" pos="relative">
      <LoadingOverlay visible={loading} overlayProps={{ blur: 2 }} />

      <Modal
        opened={modalOpened}
        onClose={closeModal}
        title="Pending Order Found"
        centered
        radius="lg"
      >
        <Text mb="md">
          You have an existing pending order. Do you want to add these items to
          it, or create a new order?
        </Text>
        <Group justify="flex-end">
          <Button variant="default" radius="md" onClick={createNewOrder}>
            Create New Order
          </Button>
          <Button color="violet" radius="md" onClick={addToPendingOrder}>
            Add to Existing Order
          </Button>
        </Group>
      </Modal>

      <Title order={2} mb="xl" fw={800}>
        Shopping Cart
        <Text span c="dimmed" size="lg" ml="sm" fw={500}>
          ({totalItems} items)
        </Text>
      </Title>

      <div className="grid grid-cols-1 lg:grid-cols-3 gap-8 items-start">
        <div className="lg:col-span-2 space-y-6">
          {items.map((item) => (
            <div
              key={item.variant.id}
              className="bg-white dark:bg-zinc-900 p-4 rounded-2xl shadow-sm border border-gray-100 dark:border-zinc-800 transition-shadow hover:shadow-md"
            >
              <Group align="start" wrap="nowrap">
                <div className="bg-gray-50 dark:bg-zinc-800 rounded-xl p-2">
                  <Image
                    src={
                      item.variant.medias && item.variant.medias.length > 0
                        ? getImageUrl(item.variant.medias[0])
                        : 'https://placehold.co/200x200/f3f4f6/a78bfa?text=Variant'
                    }
                    w={100}
                    h={100}
                    radius="md"
                    fit="contain"
                  />
                </div>
                <Stack flex={1} gap="xs">
                  <Group justify="space-between" align="start">
                    <div>
                      <Text fw={600} size="lg">
                        {item.variant.name}
                      </Text>
                      <Text size="sm" c="dimmed" lineClamp={1}>
                        {item.variant.description}
                      </Text>
                    </div>
                    <ActionIcon
                      variant="subtle"
                      color="red"
                      radius="xl"
                      onClick={() => removeFromCart(item.variant.id)}
                      className="opacity-50 hover:opacity-100 transition-opacity"
                    >
                      <Trash size={18} />
                    </ActionIcon>
                  </Group>

                  <Group justify="space-between" align="end" mt="auto">
                    <Group gap="xs">
                      <NumberInput
                        value={item.quantity}
                        onChange={(val) =>
                          updateQuantity(item.variant.id, Number(val))
                        }
                        min={1}
                        max={99}
                        allowNegative={false}
                        allowDecimal={false}
                        size="xs"
                        w={70}
                        radius="md"
                      />
                    </Group>
                    {item.variant.price && (
                      <Text fw={700} size="lg">
                        {formatPrice({
                          amount: item.variant.price.amount * item.quantity,
                          currency: item.variant.price.currency,
                        })}
                      </Text>
                    )}
                  </Group>
                </Stack>
              </Group>
            </div>
          ))}
        </div>

        <div className="lg:col-span-1 sticky top-4">
          <div className="bg-gray-50 dark:bg-zinc-900/50 p-6 rounded-3xl border border-gray-100 dark:border-zinc-800">
            <Title order={4} mb="lg" fw={700}>
              Order Summary
            </Title>
            <Stack gap="md">
              <Group justify="space-between">
                <Text c="dimmed">Total Items</Text>
                <Text fw={600}>{totalItems}</Text>
              </Group>
              <Group justify="space-between">
                <Text c="dimmed">Subtotal</Text>
                <Text fw={600}>
                  {currency
                    ? formatPrice({ amount: totalPrice, currency })
                    : `${totalPrice}`}
                </Text>
              </Group>
              <Divider color="gray.2" />
              <Group justify="space-between">
                <Text size="lg" fw={700}>
                  Total
                </Text>
                <Text size="xl" fw={800} c="violet">
                  {currency
                    ? formatPrice({ amount: totalPrice, currency })
                    : `${totalPrice}`}
                </Text>
              </Group>
              <Button
                fullWidth
                size="lg"
                radius="xl"
                color="violet"
                onClick={handleCheckout}
                disabled={items.length === 0}
                className="shadow-lg shadow-violet-500/20 hover:shadow-violet-500/40 transition-shadow"
              >
                Checkout
              </Button>
              <Button
                fullWidth
                variant="subtle"
                color="gray"
                size="sm"
                radius="xl"
                onClick={clearCart}
              >
                Clear Cart
              </Button>
            </Stack>
          </div>
        </div>
      </div>
    </Container>
  )
}
