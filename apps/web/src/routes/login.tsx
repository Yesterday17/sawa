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
import { LogIn } from 'lucide-react'

export const Route = createFileRoute('/login')({
  component: LoginPage,
})

function LoginPage() {
  const { login } = useAuth()
  const [loading, setLoading] = useState(false)
  const form = useForm({
    initialValues: {
      username: '',
      password: '',
    },
    validate: {
      username: (value) =>
        value.length < 2 ? 'Username must have at least 2 letters' : null,
      password: (value) =>
        value.length < 6 ? 'Password must have at least 6 characters' : null,
    },
  })

  const handleSubmit = async (values: typeof form.values) => {
    setLoading(true)
    try {
      await login(values)
    } catch (error) {
      console.error(error)
      form.setErrors({ username: 'Invalid username or password' })
    } finally {
      setLoading(false)
    }
  }

  return (
    <Container size={420} my={80}>
      <Stack align="center" mb={30}>
        <ThemeIcon size={60} radius="xl" variant="light" color="violet">
          <LogIn size={32} />
        </ThemeIcon>
        <Title ta="center" className="font-bold text-gray-900 dark:text-white">
          Welcome back!
        </Title>
        <Text c="dimmed" size="sm" ta="center">
          Do not have an account yet?{' '}
          <Anchor component={Link} to="/register" size="sm" c="violet" fw={500}>
            Create account
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
              Sign in
            </Button>
          </Stack>
        </form>
      </Paper>
    </Container>
  )
}
