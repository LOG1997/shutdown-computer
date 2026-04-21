import {
    Dialog,
    DialogContent,
    DialogDescription,
    DialogFooter,
    DialogHeader,
    DialogTitle,
} from "@workspace/ui/components/dialog"
import { Button } from "@workspace/ui/components/button"

interface Props {
    open: boolean;
    setOpen: (open: boolean) => void;
    children?: React.ReactNode;
    setValue?: (value: string) => void;
    handleConfirm?: () => void;
    desc?: string;
}
export default function ConfirmDialog(props: Props) {
    const { open, setOpen, handleConfirm, children, desc } = props;
    return (
        <Dialog open={open} onOpenChange={setOpen}>
            <DialogContent>
                <DialogHeader>
                    <DialogTitle>提示信息</DialogTitle>
                    <DialogDescription>
                        {desc || '确定要执行此操作吗？'}
                    </DialogDescription>
                </DialogHeader>
                <div className="py-4">
                    {children}
                </div>
                <DialogFooter>
                    <Button variant="outline" className="cursor-pointer" onClick={() => setOpen(false)}>
                        取消
                    </Button>
                    <Button className="cursor-pointer" onClick={handleConfirm}>
                        确认
                    </Button>
                </DialogFooter>
            </DialogContent>
        </Dialog>
    )
}
