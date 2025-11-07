import Image from "next/image";

export default function Home() {
  return (
    <main className="flex min-h-screen flex-col items-center justify-center p-24">
      <div className="z-10 max-w-5xl w-full items-center justify-center">
        <div className="flex justify-center mb-8">
          <Image
            src="/logo.png"
            alt="Hodei Audit Trail"
            width={200}
            height={96}
            className="h-24 w-auto"
            priority
          />
        </div>
        <h1 className="text-4xl font-bold text-center mb-4">
          Welcome to Hodei Audit
        </h1>
        <p className="text-center text-lg text-muted-foreground">
          CloudTrail-Inspired Audit Dashboard
        </p>
        <div className="mt-12 text-center">
          <p className="text-sm text-muted-foreground">
            Epic 1: Project Foundation & Setup
          </p>
        </div>
      </div>
    </main>
  );
}
