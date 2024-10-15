import { Button, Menu } from "antd";
import type { GetProp } from "antd";

type MenuItemType = GetProp<typeof Menu, "items">;

interface CronListProps {
  items: MenuItemType;
  loading: boolean;
  onClick?: (e: any) => void;
}
export default function CronList(props: CronListProps) {
  const onClick = (e: any) => {
    const job = props.items.find((item) => item!.key === e.key);
    props.onClick?.(job);
  };

  return (
    <div>
      <div className=" m-2">
        <Button type="primary" block>
          Primary
        </Button>
      </div>
      <Menu onClick={onClick} mode="inline" items={props.items} />
    </div>
  );
}
