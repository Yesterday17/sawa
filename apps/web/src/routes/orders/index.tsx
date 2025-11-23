import { createFileRoute, Link } from '@tanstack/react-router'
import { useQuery } from '@tanstack/react-query'
import { getOrdersQueryKey } from '../../client/@tanstack/react-query.gen'
import { getOrders } from '../../client/sdk.gen'
import {
  Container,
  Title,
  Table,
  Badge,
  Button,
  Group,
  Loader,
  Center,
  Text,
  Paper,
  ThemeIcon,
  ActionIcon,
  Stack,
} from '@mantine/core'
import { Plus, ShoppingBag, ChevronRight, Package } from 'lucide-react'

export const Route = createFileRoute('/orders/')({
  component: OrdersPage,
})

function OrdersPage() {
  const { data, isLoading, error } = useQuery({
    queryKey: getOrdersQueryKey({ query: { role: 'creator' } }),
    queryFn: () =>
      getOrders({ query: { role: 'creator' } }).then((res) => res.data),
  })

  if (isLoading)
    return (
      <Center h={400}>
        <Loader type="dots" />
      </Center>
    )
  if (error)
    return (
      <Center h={400}>
        <Text c="red">Error loading orders</Text>
      </Center>
    )

  return (
    <Container size="xl" py="xl">
      <Group justify="space-between" mb={40}>
        <div>
          <Title order={2} className="font-bold text-gray-900 dark:text-white">
            My Orders
          </Title>
          <Text c="dimmed" size="sm">
            Track and manage your purchases
          </Text>
        </div>
        <Button
          component={Link}
          to="/orders/create"
          leftSection={<Plus size={16} />}
          color="violet"
          className="bg-violet-600 hover:bg-violet-700 shadow-md hover:shadow-lg transition-all"
        >
          Create Order
        </Button>
      </Group>

      <Paper
        shadow="sm"
        radius="md"
        withBorder
        className="overflow-hidden bg-white dark:bg-gray-800"
      >
        <Table striped highlightOnHover verticalSpacing="md">
          <Table.Thead className="bg-gray-50 dark:bg-gray-900/50">
            <Table.Tr>
              <Table.Th>Order ID</Table.Th>
              <Table.Th>Status</Table.Th>
              <Table.Th>Total Price</Table.Th>
              <Table.Th>Items</Table.Th>
              <Table.Th style={{ width: 100 }}>Action</Table.Th>
            </Table.Tr>
          </Table.Thead>
          <Table.Tbody>
            {data?.length === 0 ? (
              <Table.Tr>
                <Table.Td colSpan={5}>
                  <Center py={60}>
                    <Stack align="center" gap="md">
                      <ThemeIcon
                        size={60}
                        radius="xl"
                        variant="light"
                        color="gray"
                      >
                        <ShoppingBag size={32} />
                      </ThemeIcon>
                      <Text c="dimmed">No orders found</Text>
                      <Button
                        component={Link}
                        to="/orders/create"
                        variant="light"
                        color="violet"
                        size="xs"
                      >
                        Start Shopping
                      </Button>
                    </Stack>
                  </Center>
                </Table.Td>
              </Table.Tr>
            ) : (
              data?.map((order) => (
                <Table.Tr key={order.id} className="group">
                  <Table.Td>
                    <Group gap="xs">
                      <ThemeIcon
                        variant="light"
                        color="gray"
                        size="sm"
                        radius="xl"
                      >
                        <Package size={12} />
                      </ThemeIcon>
                      <Text fw={500} size="sm" className="font-mono">
                        #{order.id.substring(0, 8)}
                      </Text>
                    </Group>
                  </Table.Td>
                  <Table.Td>
                    <Badge
                      color={
                        order.status === 'fulfilled'
                          ? 'green'
                          : order.status === 'cancelled'
                            ? 'red'
                            : 'blue'
                      }
                      variant="light"
                      size="sm"
                    >
                      {order.status}
                    </Badge>
                  </Table.Td>
                  <Table.Td>
                    <Text fw={600} size="sm">
                      {order.total_price
                        ? `${order.total_price.amount} ${order.total_price.currency}`
                        : '-'}
                    </Text>
                  </Table.Td>
                  <Table.Td>
                    <Text size="sm" c="dimmed">
                      {order.items.length} items
                    </Text>
                  </Table.Td>
                  <Table.Td>
                    <ActionIcon
                      component={Link}
                      to={`/orders/${order.id}`}
                      variant="subtle"
                      color="violet"
                      className="opacity-0 group-hover:opacity-100 transition-opacity"
                    >
                      <ChevronRight size={18} />
                    </ActionIcon>
                  </Table.Td>
                </Table.Tr>
              ))
            )}
          </Table.Tbody>
        </Table>
      </Paper>
    </Container>
  )
}
