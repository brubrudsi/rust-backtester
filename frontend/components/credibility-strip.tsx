import { Card } from "@/components/ui/card";
import { Badge } from "@/components/ui/badge";

const items = [
  "No lookahead",
  "Next-bar execution",
  "Costs + slippage",
  "Borrow/funding carry",
  "Adjusted prices toggle",
  "Reproducible configs",
  "Rust engine",
];

export function CredibilityStrip() {
  return (
    <Card className="p-4">
      <div className="flex flex-wrap gap-2">
        {items.map((x) => (
          <Badge key={x}>{x}</Badge>
        ))}
      </div>
    </Card>
  );
}
