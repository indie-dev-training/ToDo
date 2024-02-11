import { ToDos } from "@/components/pageContent";

export default function Page() {
  return (
    <div className="flex h-screen w-screen flex-col bg-gray-50">
      <ToDos />
    </div>
  );
}
