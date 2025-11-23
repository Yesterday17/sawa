import { createFileRoute, Link } from '@tanstack/react-router'
import {
  Container,
  Title,
  Text,
  Button,
  Group,
  Stack,
  ThemeIcon,
  SimpleGrid,
  Paper,
} from '@mantine/core'
import { useAuth } from '../context/AuthContext'
import { Package, ShieldCheck, Zap, ArrowRight } from 'lucide-react'

export const Route = createFileRoute('/')({
  component: LandingPage,
})

function LandingPage() {
  const { isAuthenticated } = useAuth()

  return (
    <Container size="xl" py={80}>
      <Stack align="center" gap={60}>
        <Stack align="center" gap="lg" className="relative">
          <div className="absolute -top-20 -left-20 w-64 h-64 bg-violet-500/20 rounded-full blur-3xl -z-10" />
          <div className="absolute -bottom-20 -right-20 w-64 h-64 bg-blue-500/20 rounded-full blur-3xl -z-10" />

          <Title
            order={1}
            size={72}
            ta="center"
            className="font-black tracking-tight bg-gradient-to-r from-violet-600 to-blue-600 bg-clip-text text-transparent"
          >
            Welcome to Sawa
          </Title>
          <Text
            size="xl"
            c="dimmed"
            ta="center"
            maw={600}
            className="text-lg md:text-xl leading-relaxed"
          >
            The premier marketplace for mystery boxes. Experience the thrill of
            discovery with our curated collections of unique items.
          </Text>

          <Group mt="xl">
            {isAuthenticated ? (
              <Button
                component={Link}
                to="/products"
                size="xl"
                radius="md"
                className="bg-violet-600 hover:bg-violet-700 shadow-lg hover:shadow-violet-500/30 transition-all hover:-translate-y-1"
                rightSection={<ArrowRight size={20} />}
              >
                Browse Collection
              </Button>
            ) : (
              <>
                <Button
                  component={Link}
                  to="/login"
                  size="xl"
                  radius="md"
                  className="bg-violet-600 hover:bg-violet-700 shadow-lg hover:shadow-violet-500/30 transition-all hover:-translate-y-1"
                >
                  Get Started
                </Button>
                <Button
                  component={Link}
                  to="/register"
                  size="xl"
                  radius="md"
                  variant="default"
                  className="hover:bg-gray-50 dark:hover:bg-gray-800 transition-all hover:-translate-y-1"
                >
                  Create Account
                </Button>
              </>
            )}
          </Group>
        </Stack>

        <SimpleGrid cols={{ base: 1, md: 3 }} spacing={30} w="100%" mt={40}>
          <FeatureCard
            icon={<Package size={32} />}
            title="Curated Mystery Boxes"
            description="Every box is packed with high-quality items selected by our expert team."
            color="violet"
          />
          <FeatureCard
            icon={<ShieldCheck size={32} />}
            title="Guaranteed Value"
            description="We ensure that the contents of every box meet or exceed the purchase price."
            color="blue"
          />
          <FeatureCard
            icon={<Zap size={32} />}
            title="Instant Delivery"
            description="Digital items are delivered instantly to your account upon purchase."
            color="teal"
          />
        </SimpleGrid>
      </Stack>
    </Container>
  )
}

function FeatureCard({
  icon,
  title,
  description,
  color,
}: {
  icon: React.ReactNode
  title: string
  description: string
  color: string
}) {
  return (
    <Paper
      p="xl"
      radius="lg"
      withBorder
      className="bg-white dark:bg-gray-800 hover:shadow-xl transition-shadow duration-300"
    >
      <ThemeIcon size={60} radius="md" variant="light" color={color} mb="lg">
        {icon}
      </ThemeIcon>
      <Title order={3} mb="sm" size="h4">
        {title}
      </Title>
      <Text c="dimmed" size="sm" lh={1.6}>
        {description}
      </Text>
    </Paper>
  )
}
