import type { Metadata } from 'next';
import { GeistSans } from 'geist/font/sans';
import { GeistMono } from 'geist/font/mono';
import { TooltipProvider } from '@/components/ui/Tooltip';
import { Toaster } from '@/components/ui/Toaster';
import './globals.css';

export const metadata: Metadata = {
  title: 'NovaNet Visualizer',
  description: 'Interactive knowledge graph visualization for NovaNet localization system',
  keywords: ['Neo4j', 'knowledge graph', 'visualization', 'localization', 'i18n'],
};

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  return (
    <html lang="en" className="dark">
      <body
        className={`${GeistSans.variable} ${GeistMono.variable} font-sans antialiased bg-black text-white`}
      >
        <TooltipProvider delayDuration={200} skipDelayDuration={100}>
          {children}
          <Toaster />
        </TooltipProvider>
      </body>
    </html>
  );
}
