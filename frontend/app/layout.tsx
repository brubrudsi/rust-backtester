import "./globals.css";
import type { Metadata } from "next";
import { Nav } from "@/components/nav";

export const metadata: Metadata = {
  title: "Rust Backtester",
  description: "Rust backtesting engine + Next.js chart-heavy UI (portfolio demo).",
};

export default function RootLayout({ children }: { children: React.ReactNode }) {
  return (
    <html lang="en">
      <body className="min-h-screen">
        <div className="mx-auto max-w-7xl px-4 py-6">
          <Nav />
          <main className="mt-6">{children}</main>
        </div>
      </body>
    </html>
  );
}
