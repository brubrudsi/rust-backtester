export function Nav() {
  return (
    <header className="flex items-center justify-between">
      <a href="/" className="no-underline">
        <div className="flex items-center">
          <div className="leading-tight">
            <div className="font-semibold">Rust Backtester</div>
            <div className="text-xs text-neutral-400">Rust engine • Next.js UI</div>
          </div>
        </div>
      </a>

      <nav className="flex items-center gap-4 text-sm">
        <a href="/engineering" className="text-neutral-300 hover:text-neutral-50 no-underline">
          Engineering
        </a>
      </nav>
    </header>
  );
}
