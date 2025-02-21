"use client"

import { Download, Upload, FileUp } from "lucide-react"
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card"
import { Input } from "@/components/ui/input"
import { Button } from "@/components/ui/button"
import { Label } from "@/components/ui/label"
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs"
import { useState } from "react"

export default function SignEscrowPage() {
  const [psbtFile, setPsbtFile] = useState<File | null>(null)
  const [psbtText, setPsbtText] = useState("")

  const handleFileUpload = (e: React.ChangeEvent<HTMLInputElement>) => {
    const file = e.target.files?.[0]
    if (file) {
      setPsbtFile(file)
      // Read file content
      const reader = new FileReader()
      reader.onload = (e) => {
        const text = e.target?.result as string
        setPsbtText(text)
      }
      reader.readAsText(file)
    }
  }

  return (
    <div className="max-w-3xl mx-auto">
      <h1 className="text-2xl font-bold mb-6">Sign Escrow</h1>
      <Card>
        <CardHeader>
          <CardTitle className="text-lg font-medium">
            Upload or paste your PSBT (Partially Signed Bitcoin Transaction)
          </CardTitle>
        </CardHeader>
        <CardContent className="p-6 space-y-6">
          <Tabs defaultValue="upload" className="space-y-4">
            <TabsList className="grid w-full grid-cols-2">
              <TabsTrigger value="upload">Upload File</TabsTrigger>
              <TabsTrigger value="paste">Paste PSBT</TabsTrigger>
            </TabsList>

            <TabsContent value="upload" className="space-y-4">
              <div className="border-2 border-dashed border-zinc-800 rounded-lg p-6 text-center">
                <Input
                  type="file"
                  accept=".psbt"
                  onChange={handleFileUpload}
                  className="hidden"
                  id="psbt-upload"
                />
                <Label
                  htmlFor="psbt-upload"
                  className="flex flex-col items-center gap-2 cursor-pointer"
                >
                  <FileUp className="h-8 w-8 text-muted-foreground" />
                  <span className="text-muted-foreground">
                    {psbtFile ? psbtFile.name : "Click to upload PSBT file"}
                  </span>
                </Label>
              </div>
              {psbtFile && (
                <div className="space-y-2">
                  <Label>File Content Preview</Label>
                  <div className="bg-zinc-900 p-3 rounded-lg font-mono text-sm break-all">
                    {psbtText.slice(0, 100)}...
                  </div>
                </div>
              )}
            </TabsContent>

            <TabsContent value="paste" className="space-y-4">
              <div className="space-y-2">
                <Label>PSBT Data</Label>
                <Input
                  placeholder="Paste your PSBT data here"
                  value={psbtText}
                  onChange={(e) => setPsbtText(e.target.value)}
                  className="font-mono"
                />
              </div>
            </TabsContent>
          </Tabs>

          <div className="space-y-4 pt-4 border-t border-zinc-800">
            <div className="flex items-center gap-2">
              <Button
                className="w-full"
                disabled={!psbtText}
                onClick={() => {
                  // Handle signing logic here
                  console.log("Signing PSBT:", psbtText)
                }}
              >
                <Download className="w-4 h-4 mr-2" />
                Sign Transaction
              </Button>
            </div>
            {psbtText && (
              <div className="text-sm text-muted-foreground">
                * Please review the transaction details carefully before signing
              </div>
            )}
          </div>
        </CardContent>
      </Card>
    </div>
  )
} 