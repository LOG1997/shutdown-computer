import { Input } from "@workspace/ui/components/input"
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
    setValue: (value: string) => void;
    handleConfirm: () => void;
}
export default function ShutDownDialog(props: Props) {
    const { open, setOpen, setValue, handleConfirm } = props;
    return (
        <Dialog open={open} onOpenChange={setOpen}>
            <DialogContent>
                <DialogHeader>
                    <DialogTitle>安全确认</DialogTitle>
                    <DialogDescription>
                        为了安全起见，请再次输入操作密码以确认关机。
                    </DialogDescription>
                </DialogHeader>
                <div className="py-4">
                    <label className="text-sm font-medium leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70 mb-2 block">
                        确认密码
                    </label>
                    <Input
                        type="password"
                        onChange={(e) => setValue(e.target.value)}
                        placeholder="请再次输入密码"
                        autoFocus // 自动聚焦方便输入
                        onKeyDown={(e) => {
                            if (e.key === 'Enter') {
                                handleConfirm()
                            }
                        }}
                    />
                </div>
                <DialogFooter>
                    <Button variant="outline" onClick={() => setOpen(false)}>
                        取消
                    </Button>
                    <Button variant="destructive" onClick={handleConfirm}>
                        确认关机
                    </Button>
                </DialogFooter>
            </DialogContent>
        </Dialog>
    )
}
