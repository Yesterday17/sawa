import {
  AppShell,
  Group,
  Title,
  Button,
  Avatar,
  Menu,
  Text,
  ThemeIcon,
  useMantineColorScheme,
  useComputedColorScheme,
  SegmentedControl,
  UnstyledButton,
  Indicator,
  Badge,
} from '@mantine/core'
import { Link, Outlet, useLocation, useNavigate } from '@tanstack/react-router'
import { useAuth } from '../context/AuthContext'
import { useCart } from '../context/CartContext'
import { useEffect } from 'react'
import { clsx } from 'clsx'
// package.json has lucide-react. I'll use lucide-react.
import {
  LogOut,
  User,
  LayoutDashboard,
  Sun,
  Moon,
  Monitor,
  Check,
  Package,
  ShoppingCart,
  Box,
  ClipboardList,
} from 'lucide-react'

export function Layout() {
  const { user, logout, isAuthenticated } = useAuth()
  const { totalItems } = useCart()
  const location = useLocation()
  const navigate = useNavigate()
  const { colorScheme, setColorScheme } = useMantineColorScheme()
  const isActive = (path: string) => location.pathname.startsWith(path)
  const computedColorScheme = useComputedColorScheme('light', {
    getInitialValueInEffect: true,
  })

  useEffect(() => {
    if (computedColorScheme === 'dark') {
      document.documentElement.classList.add('dark')
    } else {
      document.documentElement.classList.remove('dark')
    }
  }, [computedColorScheme])

  return (
    <AppShell
      header={{ height: 70 }}
      padding="md"
      className="bg-gray-50 dark:bg-[var(--mantine-color-dark-8)] transition-colors duration-300"
    >
      <AppShell.Header className="[view-transition-name:app-header] backdrop-blur-md bg-white/80 dark:bg-[var(--mantine-color-dark-7)]/80 border-b border-gray-200 dark:border-[var(--mantine-color-dark-4)] transition-colors duration-300">
        <Group h="100%" px="xl" justify="space-between">
          <Group>
            <Group gap="xs">
              <ThemeIcon size="lg" radius="md" variant="light" color="violet">
                <LayoutDashboard size={20} />
              </ThemeIcon>
              <Title
                order={3}
                className="font-black tracking-tight text-gray-900 dark:text-white"
              >
                Sawa
              </Title>
            </Group>
          </Group>

          {isAuthenticated && (
            <Group
              gap="xs"
              visibleFrom="sm"
              className="absolute left-1/2 -translate-x-1/2"
            >
              <SegmentedControl
                style={{ width: 450 }}
                value={
                  location.pathname.startsWith('/orders')
                    ? '/orders'
                    : location.pathname.startsWith('/products')
                      ? '/products'
                      : location.pathname.startsWith('/goods')
                        ? '/goods'
                        : location.pathname.startsWith('/cart')
                          ? '/cart'
                          : ''
                }
                onChange={(value) =>
                  navigate({ to: value, viewTransition: true })
                }
                data={[
                  {
                    label: (
                      <Group gap={8} justify="center" wrap="nowrap">
                        <Package size={16} />
                        <span>Products</span>
                      </Group>
                    ),
                    value: '/products',
                  },
                  {
                    label: (
                      <Group gap={8} justify="center" wrap="nowrap">
                        <ShoppingCart size={16} />
                        <span>Cart</span>
                        {totalItems > 0 && (
                          <Badge size="xs" circle color="red">
                            {totalItems}
                          </Badge>
                        )}
                      </Group>
                    ),
                    value: '/cart',
                  },
                  {
                    label: (
                      <Group gap={8} justify="center" wrap="nowrap">
                        <ClipboardList size={16} />
                        <span>Orders</span>
                      </Group>
                    ),
                    value: '/orders',
                  },
                  {
                    label: (
                      <Group gap={8} justify="center" wrap="nowrap">
                        <Box size={16} />
                        <span>Goods</span>
                      </Group>
                    ),
                    value: '/goods',
                  },
                ]}
              />
            </Group>
          )}

          <Group>
            {isAuthenticated ? (
              <Group>
                <Menu
                  shadow="md"
                  width={200}
                  position="bottom-end"
                  transitionProps={{ transition: 'pop-top-right' }}
                >
                  <Menu.Target>
                    <Button
                      variant="subtle"
                      color="gray"
                      className="hover:bg-gray-100 dark:hover:bg-[var(--mantine-color-dark-6)] transition-colors"
                      leftSection={
                        <Avatar
                          src={user?.avatar}
                          radius="xl"
                          size="sm"
                          color="initials"
                          name={user?.username}
                        />
                      }
                    >
                      <Text fw={600} size="sm">
                        {user?.username || 'User'}
                      </Text>
                    </Button>
                  </Menu.Target>

                  <Menu.Dropdown>
                    <Menu.Label>Account</Menu.Label>
                    <Menu.Item leftSection={<User size={16} />}>
                      Profile
                    </Menu.Item>

                    <Menu.Divider />

                    <Menu.Label>Theme</Menu.Label>
                    <Menu.Item
                      leftSection={<Sun size={16} />}
                      rightSection={
                        colorScheme === 'light' && <Check size={16} />
                      }
                      onClick={() => setColorScheme('light')}
                    >
                      Light
                    </Menu.Item>
                    <Menu.Item
                      leftSection={<Moon size={16} />}
                      rightSection={
                        colorScheme === 'dark' && <Check size={16} />
                      }
                      onClick={() => setColorScheme('dark')}
                    >
                      Dark
                    </Menu.Item>
                    <Menu.Item
                      leftSection={<Monitor size={16} />}
                      rightSection={
                        colorScheme === 'auto' && <Check size={16} />
                      }
                      onClick={() => setColorScheme('auto')}
                    >
                      Auto
                    </Menu.Item>

                    <Menu.Divider />
                    <Menu.Item
                      color="red"
                      leftSection={<LogOut size={16} />}
                      onClick={() => logout()}
                    >
                      Logout
                    </Menu.Item>
                  </Menu.Dropdown>
                </Menu>
              </Group>
            ) : (
              <Group>
                <Button component={Link} to="/login" variant="subtle">
                  Login
                </Button>
                <Button
                  component={Link}
                  to="/register"
                  variant="filled"
                  radius="xl"
                >
                  Get Started
                </Button>
              </Group>
            )}
          </Group>
        </Group>
      </AppShell.Header>

      <AppShell.Main className="bg-gray-50 dark:bg-[var(--mantine-color-dark-8)] min-h-screen transition-colors duration-300">
        <Outlet />
      </AppShell.Main>

      {isAuthenticated && (
        <div className="[view-transition-name:app-bottom-nav] fixed bottom-0 left-0 right-0 bg-white/80 dark:bg-[var(--mantine-color-dark-7)]/80 backdrop-blur-md border-t border-gray-200 dark:border-[var(--mantine-color-dark-4)] md:hidden z-50 pb-[env(safe-area-inset-bottom)]">
          <div className="flex justify-around p-2 gap-0">
            <UnstyledButton
              className={clsx(
                'flex flex-col items-center justify-center p-2 flex-1',
                isActive('/products')
                  ? 'text-violet-600 dark:text-violet-400'
                  : 'text-gray-500 dark:text-gray-400',
              )}
              component={Link}
              to="/products"
              viewTransition
            >
              <Package
                size={24}
                strokeWidth={isActive('/products') ? 2.5 : 2}
              />
              <Text size="xs" fw={isActive('/products') ? 600 : 400}>
                Products
              </Text>
            </UnstyledButton>
            <UnstyledButton
              className={clsx(
                'flex flex-col items-center justify-center p-2 flex-1',
                isActive('/cart')
                  ? 'text-violet-600 dark:text-violet-400'
                  : 'text-gray-500 dark:text-gray-400',
              )}
              component={Link}
              to="/cart"
              viewTransition
            >
              <Indicator
                color="red"
                label={totalItems}
                size={16}
                offset={4}
                disabled={totalItems === 0}
              >
                <ShoppingCart
                  size={24}
                  strokeWidth={isActive('/cart') ? 2.5 : 2}
                />
              </Indicator>
              <Text size="xs" fw={isActive('/cart') ? 600 : 400}>
                Cart
              </Text>
            </UnstyledButton>
            <UnstyledButton
              className={clsx(
                'flex flex-col items-center justify-center p-2 flex-1',
                isActive('/orders')
                  ? 'text-violet-600 dark:text-violet-400'
                  : 'text-gray-500 dark:text-gray-400',
              )}
              onClick={() => navigate({ to: '/orders' })}
            >
              <ClipboardList
                size={24}
                strokeWidth={isActive('/orders') ? 2.5 : 2}
              />
              <Text size="xs" fw={isActive('/orders') ? 600 : 400}>
                Orders
              </Text>
            </UnstyledButton>
            <UnstyledButton
              className={clsx(
                'flex flex-col items-center justify-center p-2 flex-1',
                isActive('/goods')
                  ? 'text-violet-600 dark:text-violet-400'
                  : 'text-gray-500 dark:text-gray-400',
              )}
              component={Link}
              to="/goods"
              viewTransition
            >
              <Box size={24} strokeWidth={isActive('/goods') ? 2.5 : 2} />
              <Text size="xs" fw={isActive('/goods') ? 600 : 400}>
                Goods
              </Text>
            </UnstyledButton>
          </div>
        </div>
      )}
    </AppShell>
  )
}
