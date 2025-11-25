import { createFileRoute, Link, useNavigate } from '@tanstack/react-router'
import {
  Container,
  Title,
  Text,
  Button,
  Group,
  Stack,
  Card,
  Image,
  ActionIcon,
  NumberInput,
  Divider,
  Checkbox,
  Modal,
  LoadingOverlay,
} from '@mantine/core'
import { useDisclosure } from '@mantine/hooks'
import { Trash, ArrowLeft, ShoppingCart } from 'lucide-react'
import { useCart } from '../context/CartContext'
import { useAuth } from '../context/AuthContext'
import { useState, useEffect } from 'react'
import { notifications } from '@mantine/notifications'
import {
  getOrders,
  postOrders,
  postOrdersByOrderIdItems,
} from '../client/sdk.gen'
import type { PurchaseOrder } from '../client/types.gen'
import { formatPrice } from '../lib/utils'

export const Route = createFileRoute('/cart')({
  component: CartPage,
})

function CartPage() {
  const { items, removeFromCart, updateQuantity, clearCart, totalItems } =
    useCart()
  const { user } = useAuth()
  const navigate = useNavigate()

  const [selectedItems, setSelectedItems] = useState<string[]>(
    items.map((i) => i.variant.id),
  )
  const [pendingOrder, setPendingOrder] = useState<PurchaseOrder | null>(null)
  const [modalOpened, { open: openModal, close: closeModal }] =
    useDisclosure(false)
  const [loading, setLoading] = useState(false)

  // Sync selection when items are removed
  useEffect(() => {
    setSelectedItems((prev) => {
      const currentIds = items.map((i) => i.variant.id)
      return prev.filter((id) => currentIds.includes(id))
    })
  }, [items])

  const toggleAll = () => {
    if (selectedItems.length === items.length) {
      setSelectedItems([])
    } else {
      setSelectedItems(items.map((i) => i.variant.id))
    }
  }

  const toggleItem = (id: string) => {
    setSelectedItems((prev) =>
      prev.includes(id) ? prev.filter((i) => i !== id) : [...prev, id],
    )
  }

  const selectedCartItems = items.filter((i) =>
    selectedItems.includes(i.variant.id),
  )

  const totalPrice = selectedCartItems.reduce((acc, item) => {
    if (item.variant.price) {
      return acc + item.variant.price.amount * item.quantity
    }
    return acc
  }, 0)

  const currency = items.find((item) => item.variant.price)?.variant.price
    ?.currency

  const handleCheckout = async () => {
    if (selectedItems.length === 0) {
      notifications.show({
        message: 'Please select items to checkout',
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
          items: selectedCartItems.map((item) => ({
            variant_id: item.variant.id,
            quantity: item.quantity,
            owner_id: user?.id,
          })),
        },
      })

      // Remove selected items from cart
      for (const id of selectedItems) {
        removeFromCart(id)
      }
      setSelectedItems([])

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
      for (const item of selectedCartItems) {
        await postOrdersByOrderIdItems({
          path: { order_id: pendingOrder.id },
          body: {
            variant_id: item.variant.id,
            quantity: item.quantity,
            owner_id: user.id,
          },
        })
      }

      for (const id of selectedItems) {
        removeFromCart(id)
      }
      setSelectedItems([])

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
          <ShoppingCart size={64} className="text-gray-300" />
          <Title order={2} c="dimmed">
            Your cart is empty
          </Title>
          <Button
            component={Link}
            to="/products"
            variant="light"
            color="violet"
            leftSection={<ArrowLeft size={16} />}
          >
            Browse Products
          </Button>
        </Stack>
      </Container>
    )
  }

  return (
    <Container size="md" py="xl" pos="relative">
      <LoadingOverlay visible={loading} overlayProps={{ blur: 2 }} />

      <Modal
        opened={modalOpened}
        onClose={closeModal}
        title="Pending Order Found"
        centered
      >
        <Text mb="md">
          You have an existing pending order. Do you want to add these items to
          it, or create a new order?
        </Text>
        <Group justify="flex-end">
          <Button variant="default" onClick={createNewOrder}>
            Create New Order
          </Button>
          <Button color="violet" onClick={addToPendingOrder}>
            Add to Existing Order
          </Button>
        </Group>
      </Modal>

      <Title order={2} mb="xl">
        Shopping Cart ({totalItems} items)
      </Title>

      <div className="grid grid-cols-1 lg:grid-cols-3 gap-8">
        <div className="lg:col-span-2 space-y-4">
          <Card withBorder padding="sm" radius="md">
            <Checkbox
              checked={
                selectedItems.length === items.length && items.length > 0
              }
              indeterminate={
                selectedItems.length > 0 && selectedItems.length < items.length
              }
              onChange={toggleAll}
              label="Select All Items"
            />
          </Card>

          {items.map((item) => (
            <Card key={item.variant.id} withBorder padding="md" radius="md">
              <Group align="start" wrap="nowrap">
                <Checkbox
                  checked={selectedItems.includes(item.variant.id)}
                  onChange={() => toggleItem(item.variant.id)}
                  mt="xl"
                />
                <Image
                  src="https://placehold.co/200x200/f3f4f6/a78bfa?text=Variant"
                  w={100}
                  h={100}
                  radius="md"
                  fit="cover"
                />
                <Stack flex={1} gap="xs">
                  <Group justify="space-between" align="start">
                    <div>
                      <Text fw={700}>{item.variant.name}</Text>
                      <Text size="sm" c="dimmed" lineClamp={1}>
                        {item.variant.description}
                      </Text>
                    </div>
                    <ActionIcon
                      variant="subtle"
                      color="red"
                      onClick={() => removeFromCart(item.variant.id)}
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
                        w={60}
                      />
                    </Group>
                    {item.variant.price && (
                      <Text fw={700}>
                        {formatPrice({
                          amount: item.variant.price.amount * item.quantity,
                          currency: item.variant.price.currency,
                        })}
                      </Text>
                    )}
                  </Group>
                </Stack>
              </Group>
            </Card>
          ))}
        </div>

        <div className="lg:col-span-1">
          <Card withBorder padding="xl" radius="md">
            <Title order={4} mb="md">
              Order Summary
            </Title>
            <Stack gap="md">
              <Group justify="space-between">
                <Text c="dimmed">Selected Items</Text>
                <Text fw={700}>{selectedCartItems.length}</Text>
              </Group>
              <Group justify="space-between">
                <Text c="dimmed">Subtotal</Text>
                <Text fw={700}>
                  {currency
                    ? formatPrice({ amount: totalPrice, currency })
                    : `${totalPrice}`}
                </Text>
              </Group>
              <Divider />
              <Group justify="space-between">
                <Text size="lg" fw={700}>
                  Total
                </Text>
                <Text size="lg" fw={700} c="violet">
                  {currency
                    ? formatPrice({ amount: totalPrice, currency })
                    : `${totalPrice}`}
                </Text>
              </Group>
              <Button
                fullWidth
                size="md"
                color="violet"
                onClick={handleCheckout}
                disabled={selectedItems.length === 0}
              >
                Checkout
              </Button>
              <Button
                fullWidth
                variant="subtle"
                color="red"
                onClick={clearCart}
              >
                Clear Cart
              </Button>
            </Stack>
          </Card>
        </div>
      </div>
    </Container>
  )
}
