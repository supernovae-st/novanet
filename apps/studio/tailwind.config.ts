import type { Config } from 'tailwindcss';

const config: Config = {
  darkMode: ['class', 'class'],
  content: [
    './src/pages/**/*.{js,ts,jsx,tsx,mdx}',
    './src/components/**/*.{js,ts,jsx,tsx,mdx}',
    './src/app/**/*.{js,ts,jsx,tsx,mdx}',
  ],
  theme: {
  	extend: {
  		colors: {
  			novanet: {
  				'50': '#f0f9ff',
  				'100': '#e0f2fe',
  				'200': '#bae6fd',
  				'300': '#7dd3fc',
  				'400': '#38bdf8',
  				'500': '#0ea5e9',
  				'600': '#0284c7',
  				'700': '#0369a1',
  				'800': '#075985',
  				'900': '#0c4a6e',
  				'950': '#082f49'
  			},
  			node: {
  				project: '#8b5cf6',
  				page: '#3b82f6',
  				block: '#06b6d4',
  				blockType: '#14b8a6',
  				concept: '#f59e0b',
  				conceptL10n: '#fbbf24',
  				locale: '#10b981',
  				sharedLayer: '#22c55e',
  				expression: '#ec4899'
  			},
  			glass: {
  				bg: 'rgba(0, 0, 0, 0.6)',
  				border: 'rgba(255, 255, 255, 0.1)',
  				hover: 'rgba(255, 255, 255, 0.05)'
  			},
  			background: 'hsl(var(--background))',
  			foreground: 'hsl(var(--foreground))',
  			card: {
  				DEFAULT: 'hsl(var(--card))',
  				foreground: 'hsl(var(--card-foreground))'
  			},
  			popover: {
  				DEFAULT: 'hsl(var(--popover))',
  				foreground: 'hsl(var(--popover-foreground))'
  			},
  			primary: {
  				DEFAULT: 'hsl(var(--primary))',
  				foreground: 'hsl(var(--primary-foreground))'
  			},
  			secondary: {
  				DEFAULT: 'hsl(var(--secondary))',
  				foreground: 'hsl(var(--secondary-foreground))'
  			},
  			muted: {
  				DEFAULT: 'hsl(var(--muted))',
  				foreground: 'hsl(var(--muted-foreground))'
  			},
  			accent: {
  				DEFAULT: 'hsl(var(--accent))',
  				foreground: 'hsl(var(--accent-foreground))',
  				blue: 'hsl(var(--accent-blue))',
  				orange: 'hsl(var(--accent-orange))',
  				green: 'hsl(var(--accent-green))',
  				red: 'hsl(var(--accent-red))',
  				yellow: 'hsl(var(--accent-yellow))',
  				purple: 'hsl(var(--accent-purple))'
  			},
  			destructive: {
  				DEFAULT: 'hsl(var(--destructive))',
  				foreground: 'hsl(var(--destructive-foreground))'
  			},
  			border: 'hsl(var(--border))',
  			input: 'hsl(var(--input))',
  			ring: 'hsl(var(--ring))',
  			chart: {
  				'1': 'hsl(var(--chart-1))',
  				'2': 'hsl(var(--chart-2))',
  				'3': 'hsl(var(--chart-3))',
  				'4': 'hsl(var(--chart-4))',
  				'5': 'hsl(var(--chart-5))'
  			}
  		},
  		backgroundImage: {
  			'gradient-radial': 'radial-gradient(var(--tw-gradient-stops))',
  			'gradient-conic': 'conic-gradient(from 180deg at 50% 50%, var(--tw-gradient-stops))',
  			'glass-gradient': 'linear-gradient(135deg, rgba(255,255,255,0.1) 0%, rgba(255,255,255,0.05) 100%)'
  		},
  		backdropBlur: {
  			xs: '2px'
  		},
  		animation: {
  			'pulse-slow': 'pulse 3s cubic-bezier(0.4, 0, 0.6, 1) infinite',
  			glow: 'glow 2s ease-in-out infinite alternate',
  			shake: 'shake 0.5s cubic-bezier(.36,.07,.19,.97) both',
  			'toast-enter': 'toast-enter 0.3s ease-out',
  			'toast-exit': 'toast-exit 0.3s ease-in forwards',
  			'selection-ping': 'selection-ping 3s cubic-bezier(0, 0, 0.2, 1) infinite',
  			'selection-ping-delayed': 'selection-ping 3s cubic-bezier(0, 0, 0.2, 1) infinite 1s',
  			'shimmer': 'shimmer 8s ease-in-out infinite',
  			'float': 'float 6s ease-in-out infinite',
  			'glow-pulse-selected': 'glow-pulse-selected 4s ease-in-out infinite',
  			'badge-in': 'badgeIn 200ms ease-out both'
  		},
  		keyframes: {
  			glow: {
  				'0%': {
  					boxShadow: '0 0 5px currentColor, 0 0 10px currentColor'
  				},
  				'100%': {
  					boxShadow: '0 0 10px currentColor, 0 0 20px currentColor, 0 0 30px currentColor'
  				}
  			},
  			shake: {
  				'10%, 90%': {
  					transform: 'translate3d(-1px, 0, 0)'
  				},
  				'20%, 80%': {
  					transform: 'translate3d(2px, 0, 0)'
  				},
  				'30%, 50%, 70%': {
  					transform: 'translate3d(-4px, 0, 0)'
  				},
  				'40%, 60%': {
  					transform: 'translate3d(4px, 0, 0)'
  				}
  			},
  			'toast-enter': {
  				'0%': {
  					opacity: '0',
  					transform: 'translateY(10px) scale(0.95)'
  				},
  				'100%': {
  					opacity: '1',
  					transform: 'translateY(0) scale(1)'
  				}
  			},
  			'toast-exit': {
  				'0%': {
  					opacity: '1',
  					transform: 'translateY(0) scale(1)'
  				},
  				'100%': {
  					opacity: '0',
  					transform: 'translateY(-10px) scale(0.95)'
  				}
  			},
  			'selection-ping': {
  				'0%': {
  					transform: 'scale(1)',
  					opacity: '0.5'
  				},
  				'50%': {
  					transform: 'scale(1.08)',
  					opacity: '0.25'
  				},
  				'100%': {
  					transform: 'scale(1.15)',
  					opacity: '0'
  				}
  			},
  			'shimmer': {
  				'0%': {
  					backgroundPosition: '-200% 0'
  				},
  				'100%': {
  					backgroundPosition: '200% 0'
  				}
  			},
  			'float': {
  				'0%, 100%': {
  					transform: 'translateY(0)'
  				},
  				'50%': {
  					transform: 'translateY(-2px)'
  				}
  			},
  			'glow-pulse-selected': {
  				'0%, 100%': {
  					opacity: '0.7',
  					transform: 'scale(1)'
  				},
  				'50%': {
  					opacity: '1',
  					transform: 'scale(1.01)'
  				}
  			},
  			badgeIn: {
  				'0%': {
  					opacity: '0',
  					transform: 'translateX(-8px) scale(0.95)'
  				},
  				'100%': {
  					opacity: '1',
  					transform: 'translateX(0) scale(1)'
  				}
  			}
  		},
  		fontFamily: {
  			sans: [
  				'var(--font-geist-sans)'
  			],
  			mono: [
  				'var(--font-geist-mono)'
  			]
  		},
  		borderRadius: {
  			lg: 'var(--radius)',
  			md: 'calc(var(--radius) - 2px)',
  			sm: 'calc(var(--radius) - 4px)'
  		}
  	}
  },
  plugins: [
    require('@tailwindcss/typography'),
      require("tailwindcss-animate")
],
};

export default config;
