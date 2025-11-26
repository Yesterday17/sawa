import { createTheme, rem, type MantineTheme } from '@mantine/core'

export const theme = createTheme({
  primaryColor: 'violet',
  fontFamily:
    'Inter, -apple-system, BlinkMacSystemFont, Segoe UI, Roboto, Helvetica, Arial, sans-serif',
  defaultRadius: 'lg',
  shadows: {
    xs: '0 1px 2px 0 rgba(0, 0, 0, 0.05)',
    sm: '0 1px 3px 0 rgba(0, 0, 0, 0.1), 0 1px 2px -1px rgba(0, 0, 0, 0.1)',
    md: '0 4px 6px -1px rgba(0, 0, 0, 0.1), 0 2px 4px -1px rgba(0, 0, 0, 0.06)',
    lg: '0 10px 15px -3px rgba(0, 0, 0, 0.1), 0 4px 6px -2px rgba(0, 0, 0, 0.05)',
    xl: '0 20px 25px -5px rgba(0, 0, 0, 0.1), 0 10px 10px -5px rgba(0, 0, 0, 0.04)',
    xxl: '0 25px 50px -12px rgba(0, 0, 0, 0.25)',
  },
  headings: {
    fontFamily:
      'Inter, -apple-system, BlinkMacSystemFont, Segoe UI, Roboto, Helvetica, Arial, sans-serif',
    sizes: {
      h1: { fontSize: rem(36), fontWeight: '900', lineHeight: '1.1' },
      h2: { fontSize: rem(30), fontWeight: '800', lineHeight: '1.2' },
      h3: { fontSize: rem(24), fontWeight: '700', lineHeight: '1.3' },
    },
  },
  components: {
    Button: {
      defaultProps: {
        size: 'md',
        radius: 'xl',
      },
      styles: {
        root: {
          transition: 'all 0.2s ease',
          '&:active': {
            transform: 'translateY(1px)',
          },
        },
      },
    },
    ActionIcon: {
      defaultProps: {
        radius: 'xl',
      },
      styles: {
        root: {
          transition: 'all 0.2s ease',
        },
      },
    },
    Card: {
      defaultProps: {
        shadow: 'sm',
        withBorder: true,
        radius: 'lg',
      },
      styles: (theme: MantineTheme) => ({
        root: {
          transition: 'all 0.3s ease',
          backgroundColor: 'var(--mantine-color-body)',
        },
      }),
    },
    Paper: {
      defaultProps: {
        radius: 'lg',
      },
    },
    Modal: {
      defaultProps: {
        radius: 'lg',
        overlayProps: {
          blur: 4,
        },
      },
    },
    Container: {
      defaultProps: {
        size: 'xl',
      },
    },
  },
})
