import "./globals.css";
import type { Metadata } from "next";
import { Nav } from "@/components/nav";
import { Plus_Jakarta_Sans } from "next/font/google";

export const metadata: Metadata = {
  title: "Rust Backtester",
  description: "Rust backtesting engine + Next.js chart-heavy UI (portfolio demo).",
};

const plusJakarta = Plus_Jakarta_Sans({
  subsets: ["latin"],
  variable: "--font-sans",
  display: "swap",
});

export default function RootLayout({ children }: { children: React.ReactNode }) {
  return (
    <html lang="en" className={plusJakarta.variable}>
      <body className="min-h-screen font-sans antialiased">
        <div className="mx-auto max-w-7xl px-4 py-6 flex flex-col min-h-screen">
          <Nav />
          <main className="mt-6 flex-1 flex flex-col">{children}</main>
        </div>
      </body>
    </html>
  );
}
