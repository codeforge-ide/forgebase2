import type { Metadata } from 'next'
import { MuiProvider } from '@/components/MuiProvider'

export const metadata: Metadata = {
  title: 'ForgeBase - Open Source BaaS',
  description: 'The modern, open-source Firebase alternative built in Rust',
}

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode
}>) {
  return (
    <html lang="en">
      <body>
        <MuiProvider>{children}</MuiProvider>
      </body>
    </html>
  )
}
