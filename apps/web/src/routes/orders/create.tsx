import { createFileRoute, useNavigate } from '@tanstack/react-router'
import { useState } from 'react'
import {
  Stepper,
  Button,
  Group,
  Container,
  Title,
  Paper,
  Text,
  Select,
  NumberInput,
} from '@mantine/core'
import { useForm } from '@mantine/form'
import { useMutation, useQuery } from '@tanstack/react-query'
import {
  postOrdersMutation,
  getProductsVariantsOptions,
} from '../../client/@tanstack/react-query.gen'
import type { CreateOrderItemBody } from '../../client/types.gen'

export const Route = createFileRoute('/orders/create')({
  component: CreateOrderPage,
})

function CreateOrderPage() {
  const [active, setActive] = useState(0)
  const navigate = useNavigate()
  const createOrderMutation = useMutation(postOrdersMutation())

  // Fetch variants to select from
  const { data: variants } = useQuery(getProductsVariantsOptions())

  const form = useForm({
    initialValues: {
      receiver_id: '',
      items: [] as CreateOrderItemBody[],
    },
  })

  // Temporary state for adding an item
  const [selectedVariant, setSelectedVariant] = useState<string | null>(null)
  const [quantity, setQuantity] = useState<number>(1)

  const nextStep = () =>
    setActive((current) => (current < 2 ? current + 1 : current))
  const prevStep = () =>
    setActive((current) => (current > 0 ? current - 1 : current))

  const addItem = () => {
    if (selectedVariant && quantity > 0) {
      form.insertListItem('items', {
        variant_id: selectedVariant,
        quantity: quantity,
      })
      setSelectedVariant(null)
      setQuantity(1)
    }
  }

  const handleSubmit = async () => {
    try {
      await createOrderMutation.mutateAsync({
        body: {
          items: form.values.items,
          // receiver_id: form.values.receiver_id || undefined, // TODO: Handle receiver
        },
      })
      navigate({ to: '/orders' })
    } catch (error) {
      console.error(error)
    }
  }

  const variantOptions =
    variants?.map((v) => ({ value: v.id, label: v.name })) || []

  return (
    <Container size="md" my={40}>
      <Title mb="xl">Create New Order</Title>

      <Stepper active={active} onStepClick={setActive}>
        <Stepper.Step label="Receiver" description="Who is this for?">
          <Paper withBorder p="md" mt="md">
            <Text>For now, creating order for yourself.</Text>
            {/* TODO: Implement receiver selection */}
          </Paper>
        </Stepper.Step>

        <Stepper.Step label="Items" description="Add items">
          <Paper withBorder p="md" mt="md">
            <Group align="flex-end">
              <Select
                label="Select Product"
                className="flex-1"
                placeholder="Pick one"
                data={variantOptions}
                value={selectedVariant}
                onChange={setSelectedVariant}
                searchable
              />
              <NumberInput
                label="Quantity"
                value={quantity}
                onChange={(val) => setQuantity(Number(val))}
                min={1}
                w={100}
              />
              <Button onClick={addItem} disabled={!selectedVariant}>
                Add
              </Button>
            </Group>

            <Title order={5} mt="lg">
              Items in Order
            </Title>
            {form.values.items.length === 0 && (
              <Text c="dimmed">No items added yet.</Text>
            )}
            {form.values.items.map((item, index) => {
              const variant = variants?.find((v) => v.id === item.variant_id)
              return (
                <Paper
                  key={[item.variant_id, index].join('-')}
                  withBorder
                  p="xs"
                  mt="xs"
                >
                  <Group justify="space-between">
                    <Text>{variant?.name || item.variant_id}</Text>
                    <Text>x{item.quantity}</Text>
                  </Group>
                </Paper>
              )
            })}
          </Paper>
        </Stepper.Step>

        <Stepper.Step label="Review" description="Review and create">
          <Paper withBorder p="md" mt="md">
            <Text>Items count: {form.values.items.length}</Text>
          </Paper>
        </Stepper.Step>

        <Stepper.Completed>
          <Paper withBorder p="md" mt="md">
            <Text>Order is ready to be created!</Text>
          </Paper>
        </Stepper.Completed>
      </Stepper>

      <Group justify="center" mt="xl">
        <Button variant="default" onClick={prevStep} disabled={active === 0}>
          Back
        </Button>
        {active < 2 ? (
          <Button onClick={nextStep}>Next step</Button>
        ) : (
          <Button
            onClick={handleSubmit}
            loading={createOrderMutation.isPending}
          >
            Create Order
          </Button>
        )}
      </Group>
    </Container>
  )
}
