import { createFileRoute } from '@tanstack/react-router'
import { useQuery } from '@tanstack/react-query'
import { getGoodsByQueryByOptions } from '../../client/@tanstack/react-query.gen'
import {
  SimpleGrid,
  Card,
  Text,
  Group,
  Container,
  Title,
  Loader,
  Center,
  Stack,
  Badge,
} from '@mantine/core'

export const Route = createFileRoute('/goods/')({
  component: GoodsPage,
})

function GoodsPage() {
  const {
    data: instances,
    isLoading,
    error,
  } = useQuery(
    getGoodsByQueryByOptions({
      path: { query_by: 'owned' },
    }),
  )

  if (isLoading)
    return (
      <Center h={400}>
        <Loader type="dots" />
      </Center>
    )
  if (error)
    return (
      <Center h={400}>
        <Text c="red">Error loading goods</Text>
      </Center>
    )

  return (
    <Container size="md" py="xl">
      <Group
        justify="space-between"
        mb={40}
        className="[view-transition-name:goods-header]"
      >
        <div>
          <Title order={2}>Goods</Title>
          <Text c="dimmed" size="sm">
            Your inventory
          </Text>
        </div>
      </Group>

      <SimpleGrid cols={{ base: 1, sm: 2, md: 3 }} spacing="md">
        {instances?.map((instance) => (
          <Card key={instance.id} withBorder padding="md" radius="md">
            <Stack gap="xs">
              <Group justify="space-between">
                <Text fw={700} size="sm" c="dimmed">
                  ID: {instance.id.slice(0, 8)}...
                </Text>
                <Badge
                  color={
                    instance.status === 'active'
                      ? 'green'
                      : instance.status === 'locked'
                        ? 'yellow'
                        : 'gray'
                  }
                >
                  {instance.status}
                </Badge>
              </Group>
              <Text size="xs" c="dimmed">
                Variant ID: {instance.variant_id}
              </Text>
              <Text size="xs" c="dimmed">
                Created: {new Date(instance.created_at).toLocaleDateString()}
              </Text>
            </Stack>
          </Card>
        ))}
      </SimpleGrid>
    </Container>
  )
}
