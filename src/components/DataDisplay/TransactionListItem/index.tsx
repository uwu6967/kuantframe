import { Group, Paper, Stack, Text, Tooltip } from "@mantine/core";
import { TauriTypes } from "$types";
import dayjs from "dayjs";
import classes from "./TransactionListItem.module.css";
import { ItemName } from "../ItemName";
import { useTranslateComponent } from "@hooks/useTranslate.hook";

export type TransactionListItemProps = {
  transaction: TauriTypes.TransactionDto;
  orientation?: "horizontal" | "vertical";
};

type TransactionMetrics = {
  sold?: number;
  bought?: number;
  profit?: number;
};

function asFiniteNumber(value: unknown): number | undefined {
  return typeof value === "number" && Number.isFinite(value) ? value : undefined;
}

export function getTransactionMetrics(transaction: TauriTypes.TransactionDto): TransactionMetrics {
  const storedPurchase = asFiniteNumber(transaction.properties?.purchase_price);
  const isSale = transaction.transaction_type === "sale";

  if (isSale) {
    const profit = transaction.profit ?? undefined;
    const bought = storedPurchase ?? (profit != null ? transaction.price - profit : undefined);
    return { sold: transaction.price, bought, profit };
  }

  return { bought: transaction.price };
}

function formatProfit(profit: number): string {
  return profit > 0 ? `+${profit}` : `${profit}`;
}

function TransactionMetric({
  label,
  value,
  color,
  suffix = "",
}: {
  label: string;
  value: number;
  color: string;
  suffix?: string;
}) {
  return (
    <Tooltip label={label} withArrow>
      <div className={classes.metric}>
        <Text className={classes.label} span>
          {label}
        </Text>
        <Text className={classes.value} c={color} span>
          {value}
          {suffix}
        </Text>
      </div>
    </Tooltip>
  );
}

function TransactionMetricsView({ transaction, compact }: { transaction: TauriTypes.TransactionDto; compact?: boolean }) {
  const soldLabel = useTranslateComponent("transaction_list_item.sold");
  const boughtLabel = useTranslateComponent("transaction_list_item.bought");
  const profitLabel = useTranslateComponent("transaction_list_item.profit");
  const metrics = getTransactionMetrics(transaction);
  const profitColor = metrics.profit == null ? "gray.4" : metrics.profit >= 0 ? "green.5" : "red.5";
  const suffix = compact ? "p" : "";

  return (
    <div className={classes.metrics} data-compact={compact || undefined}>
      {metrics.sold != null && <TransactionMetric label={soldLabel} value={metrics.sold} color="blue.5" suffix={suffix} />}
      {metrics.bought != null && <TransactionMetric label={boughtLabel} value={metrics.bought} color="orange.4" suffix={suffix} />}
      {metrics.profit != null && (
        <Tooltip label={profitLabel} withArrow>
          <div className={classes.metric}>
            <Text className={classes.label} span>
              {profitLabel}
            </Text>
            <Text className={classes.value} c={profitColor} span>
              {formatProfit(metrics.profit)}
              {suffix}
            </Text>
          </div>
        </Tooltip>
      )}
    </div>
  );
}

export function TransactionListItem({ transaction, orientation = "horizontal" }: TransactionListItemProps) {
  return (
    <Paper mt={5} classNames={{ root: classes.root }} p={5} data-transaction-type={transaction.transaction_type} data-color-mode="box-shadow">
      {orientation === "horizontal" && (
        <Group justify="space-between" wrap="nowrap" gap="sm">
          <Group ml={10} gap="sm" className={classes.name}>
            <ItemName color="gray.4" size="md" value={transaction} />
          </Group>
          <TransactionMetricsView transaction={transaction} />
          <Text className={classes.date} c="gray.4">
            {dayjs(transaction.created_at).format("DD/MM/YYYY HH:mm:ss")}
          </Text>
        </Group>
      )}
      {orientation === "vertical" && (
        <Stack gap={4}>
          <Group ml={10} gap="sm" justify="space-between" wrap="nowrap">
            <ItemName color="gray.4" size="md" value={transaction} />
          </Group>
          <Group ml={10} justify="space-between" wrap="nowrap">
            <TransactionMetricsView transaction={transaction} compact />
            <Text c="gray.4">{dayjs(transaction.created_at).format("DD/MM/YYYY HH:mm:ss")}</Text>
          </Group>
        </Stack>
      )}
    </Paper>
  );
}

