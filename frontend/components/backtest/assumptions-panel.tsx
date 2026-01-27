import type { Assumptions } from "@/lib/types";
import { Separator } from "@/components/ui/separator";

export function AssumptionsPanel({ assumptions }: { assumptions: Assumptions }) {
  const rows = [
    ["Execution", assumptions.execution],
    ["Fees", assumptions.fees],
    ["Slippage", assumptions.slippage],
    ["Borrow", assumptions.borrow],
    ["Funding", assumptions.funding],
    ["Adjusted prices", assumptions.adjusted_prices],
    ["Missing data", assumptions.missing_data],
    ["Timezone", assumptions.timezone],
  ];

  return (
    <div className="space-y-3">
      <div className="text-sm font-semibold">Assumptions</div>
      <Separator />
      <div className="space-y-2 text-sm">
        {rows.map(([k, v]) => (
          <div key={k} className="grid grid-cols-3 gap-3">
            <div className="text-neutral-400">{k}</div>
            <div className="col-span-2 text-neutral-200">{v}</div>
          </div>
        ))}
      </div>
    </div>
  );
}
