import { createTheme, rem, type MantineTheme } from '@mantine/core'

export const theme = createTheme({
  primaryColor: 'violet',
  fontFamily:
    'Inter, -apple-system, BlinkMacSystemFont, Segoe UI, Roboto, Helvetica, Arial, sans-serif',
  defaultRadius: 'md',
  shadows: {
    md: '0 4px 6px -1px rgba(0, 0, 0, 0.1), 0 2px 4px -1px rgba(0, 0, 0, 0.06)',
    lg: '0 10px 15px -3px rgba(0, 0, 0, 0.1), 0 4px 6px -2px rgba(0, 0, 0, 0.05)',
    xl: '0 20px 25px -5px rgba(0, 0, 0, 0.1), 0 10px 10px -5px rgba(0, 0, 0, 0.04)',
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
      },
      styles: {
        root: {
          transition:
            'background-color 0.2s ease, color 0.2s ease, border-color 0.2s ease, transform 0.1s ease',
          '&:active': {
            transform: 'translateY(1px)',
          },
        },
      },
    },
    ActionIcon: {
      styles: {
        root: {
          transition:
            'background-color 0.2s ease, color 0.2s ease, border-color 0.2s ease',
        },
      },
    },
    NavLink: {
      styles: {
        root: {
          transition: 'background-color 0.2s ease, color 0.2s ease',
        },
      },
    },
    MenuItem: {
      styles: {
        item: {
          transition: 'background-color 0.2s ease, color 0.2s ease',
        },
      },
    },
    Card: {
      defaultProps: {
        shadow: 'sm',
        withBorder: true,
      },
      styles: (theme: MantineTheme) => ({
        root: {
          transition:
            'transform 0.2s ease, box-shadow 0.2s ease, border-color 0.2s ease, background-color 0.2s ease',
          '&:hover': {
            transform: 'translateY(-4px)',
            boxShadow: theme.shadows.lg,
          },
        },
      }),
    },
    Container: {
      defaultProps: {
        size: 'xl',
      },
    },
  },
})
