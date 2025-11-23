import { createFileRoute, Link } from '@tanstack/react-router'
import {
  TextInput,
  PasswordInput,
  Button,
  Paper,
  Title,
  Container,
  Text,
  Anchor,
  Stack,
  ThemeIcon,
} from '@mantine/core'
import { useForm } from '@mantine/form'
import { useAuth } from '../context/AuthContext'
import { useState } from 'react'
import { UserPlus } from 'lucide-react'

export const Route = createFileRoute('/register')({
  component: RegisterPage,
})

function RegisterPage() {
  const { register } = useAuth()
  const [loading, setLoading] = useState(false)
  const form = useForm({
    initialValues: {
      email: '',
      username: '',
      password: '',
    },
    validate: {
      email: (value) => (/^\S+@\S+$/.test(value) ? null : 'Invalid email'),
      username: (value) =>
        value.length < 2 ? 'Username must have at least 2 letters' : null,
      password: (value) =>
        value.length < 6 ? 'Password must have at least 6 characters' : null,
    },
  })

  const handleSubmit = async (values: typeof form.values) => {
    setLoading(true)
    try {
      await register(values)
    } catch (error) {
      console.error(error)
      // Handle error properly
    } finally {
      setLoading(false)
    }
  }

  return (
    <Container size={420} my={80}>
      <Stack align="center" mb={30}>
        <ThemeIcon size={60} radius="xl" variant="light" color="violet">
          <UserPlus size={32} />
        </ThemeIcon>
        <Title ta="center" className="font-bold text-gray-900 dark:text-white">
          Create an account
        </Title>
        <Text c="dimmed" size="sm" ta="center">
          Already have an account?{' '}
          <Anchor component={Link} to="/login" size="sm" c="violet" fw={500}>
            Login
          </Anchor>
        </Text>
      </Stack>

      <Paper
        withBorder
        shadow="xl"
        p={30}
        radius="md"
        className="bg-white dark:bg-gray-800"
      >
        <form onSubmit={form.onSubmit(handleSubmit)}>
          <Stack gap="md">
            <TextInput
              label="Email"
              placeholder="you@example.com"
              required
              classNames={{ input: 'focus:border-violet-500' }}
              {...form.getInputProps('email')}
            />
            <TextInput
              label="Username"
              placeholder="Your username"
              required
              classNames={{ input: 'focus:border-violet-500' }}
              {...form.getInputProps('username')}
            />
            <PasswordInput
              label="Password"
              placeholder="Your password"
              required
              classNames={{ input: 'focus:border-violet-500' }}
              {...form.getInputProps('password')}
            />
            <Button
              fullWidth
              mt="md"
              type="submit"
              loading={loading}
              color="violet"
              className="bg-violet-600 hover:bg-violet-700 transition-colors duration-200"
            >
              Register
            </Button>
          </Stack>
        </form>
      </Paper>
    </Container>
  )
}
