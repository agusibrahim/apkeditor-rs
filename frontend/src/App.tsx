import { useState, useCallback, useEffect, useRef } from 'react';
import { Button } from '@/components/ui/button';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { Input } from '@/components/ui/input';
import { Label } from '@/components/ui/label';
import { Badge } from '@/components/ui/badge';
import { Progress } from '@/components/ui/progress';
import { Alert, AlertDescription, AlertTitle } from '@/components/ui/alert';
import { Checkbox } from '@/components/ui/checkbox';
import {
  Upload, FileArchive, X, Loader2, Pencil, Package, Type, Hash, Tag,
  Lock, Key, Check, Sparkles, AlertCircle, Smartphone, Moon, Sun, Monitor, Github, Shield, Zap
} from 'lucide-react';
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
} from '@/components/ui/dropdown-menu';
import { toast } from 'sonner';
import { cn } from '@/lib/utils';
import { useDevice } from '@/hooks/use-device';
import { useWasm, useApk, type WasmModule, type ApkFile, type ApkInfo } from '@/hooks/use-wasm';

// ============ TYPES ============
type Theme = 'light' | 'dark' | 'system';

interface ManifestValues {
  packageName: string;
  appName: string;
  versionCode: string;
  versionName: string;
}

interface SigningConfig {
  useCustomKey: boolean;
  keystoreData: Uint8Array | null;
  keystorePassword: string;
  keystoreFileName: string;
  aliases: string[];
  selectedAlias: string;
}

// ============ UTILITIES ============
function formatFileSize(bytes: number): string {
  if (bytes < 1024) return bytes + ' B';
  if (bytes < 1024 * 1024) return (bytes / 1024).toFixed(1) + ' KB';
  return (bytes / (1024 * 1024)).toFixed(2) + ' MB';
}

// ============ HEADER COMPONENT ============
function Header({ version, isMobile = false }: { version: string | null; isMobile?: boolean }) {
  const [theme, setTheme] = useState<Theme>('dark');

  useEffect(() => {
    const saved = localStorage.getItem('theme') as Theme | null;
    if (saved) {
      setTheme(saved);
      applyTheme(saved);
    }
  }, []);

  const applyTheme = (newTheme: Theme) => {
    const isDark = newTheme === 'dark' || (newTheme === 'system' && window.matchMedia('(prefers-color-scheme: dark)').matches);
    document.documentElement.classList.toggle('dark', isDark);
  };

  const handleThemeChange = (newTheme: Theme) => {
    setTheme(newTheme);
    localStorage.setItem('theme', newTheme);
    applyTheme(newTheme);
  };

  if (isMobile) {
    return (
      <header className="sticky top-0 z-50 glass border-b border-border/40 safe-top">
        <div className="flex items-center justify-between px-4 py-3">
          <div className="flex items-center gap-2">
            <Smartphone className="h-6 w-6 text-primary" />
            <h1 className="text-lg font-bold gradient-text">APK Editor</h1>
          </div>
          <div className="flex items-center gap-2">
            {version && <Badge variant="outline" className="text-xs">v{version}</Badge>}
            <ThemeToggle theme={theme} onThemeChange={handleThemeChange} />
          </div>
        </div>
      </header>
    );
  }

  return (
    <header className="sticky top-0 z-50 glass border-b border-border/40">
      <div className="container mx-auto flex items-center justify-between px-6 py-4">
        <div className="flex items-center gap-4">
          <div className="flex items-center gap-3">
            <div className="relative">
              <Smartphone className="h-8 w-8 text-primary" />
              <div className="absolute -bottom-1 -right-1 h-3 w-3 rounded-full bg-green-500 border-2 border-background" />
            </div>
            <div>
              <h1 className="text-2xl font-bold gradient-text">APK Editor</h1>
              <p className="text-xs text-muted-foreground">Edit & Sign APKs in Browser</p>
            </div>
          </div>
          {version && <Badge variant="secondary" className="ml-2">v{version}</Badge>}
          <Badge className="bg-gradient-to-r from-primary to-purple-500 text-primary-foreground">Rust + WASM</Badge>
        </div>
        <div className="flex items-center gap-3">
          <Button variant="ghost" size="icon" asChild>
            <a href="https://github.com/agusibrahim/apkeditor-rs" target="_blank" rel="noopener noreferrer">
              <Github className="h-5 w-5" />
            </a>
          </Button>
          <ThemeToggle theme={theme} onThemeChange={handleThemeChange} />
        </div>
      </div>
    </header>
  );
}

function ThemeToggle({ onThemeChange }: { theme: Theme; onThemeChange: (theme: Theme) => void }) {
  return (
    <DropdownMenu>
      <DropdownMenuTrigger asChild>
        <Button variant="outline" size="icon">
          <Sun className="h-4 w-4 rotate-0 scale-100 transition-all dark:-rotate-90 dark:scale-0" />
          <Moon className="absolute h-4 w-4 rotate-90 scale-0 transition-all dark:rotate-0 dark:scale-100" />
          <span className="sr-only">Toggle theme</span>
        </Button>
      </DropdownMenuTrigger>
      <DropdownMenuContent align="end">
        <DropdownMenuItem onClick={() => onThemeChange('light')}><Sun className="mr-2 h-4 w-4" />Light</DropdownMenuItem>
        <DropdownMenuItem onClick={() => onThemeChange('dark')}><Moon className="mr-2 h-4 w-4" />Dark</DropdownMenuItem>
        <DropdownMenuItem onClick={() => onThemeChange('system')}><Monitor className="mr-2 h-4 w-4" />System</DropdownMenuItem>
      </DropdownMenuContent>
    </DropdownMenu>
  );
}

// ============ FOOTER COMPONENT ============
function Footer({ isMobile = false }: { isMobile?: boolean }) {
  if (isMobile) {
    return (
      <footer className="border-t border-border/40 bg-muted/30 px-4 py-4 safe-bottom">
        <div className="flex flex-col items-center gap-2 text-center text-xs text-muted-foreground">
          <div className="flex items-center gap-4">
            <span className="flex items-center gap-1"><Zap className="h-3 w-3 text-primary" />Rust + WASM</span>
            <span className="flex items-center gap-1"><Shield className="h-3 w-3 text-green-500" />Offline</span>
          </div>
          <p>All processing happens in your browser</p>
        </div>
      </footer>
    );
  }

  return (
    <footer className="border-t border-border/40 bg-muted/30">
      <div className="container mx-auto px-6 py-6">
        <div className="flex flex-col items-center justify-between gap-4 md:flex-row">
          <div className="flex items-center gap-6 text-sm text-muted-foreground">
            <span className="flex items-center gap-2"><Zap className="h-4 w-4 text-primary" />Built with Rust & WebAssembly</span>
            <span className="flex items-center gap-2"><Shield className="h-4 w-4 text-green-500" />100% Offline - No data uploaded</span>
          </div>
          <a href="https://github.com/agusibrahim/apkeditor-rs" target="_blank" rel="noopener noreferrer" className="flex items-center gap-2 text-sm text-muted-foreground hover:text-foreground transition-colors">
            <Github className="h-4 w-4" />View on GitHub
          </a>
        </div>
      </div>
    </footer>
  );
}

// ============ FILE DROPZONE ============
function FileDropzone({ onFileSelect, apk, isLoading, onClear, isMobile = false }: {
  onFileSelect: (file: File) => void;
  apk: ApkFile | null;
  isLoading: boolean;
  onClear: () => void;
  isMobile?: boolean;
}) {
  const [isDragging, setIsDragging] = useState(false);
  const fileInputRef = useRef<HTMLInputElement>(null);

  const handleDragOver = useCallback((e: React.DragEvent) => { e.preventDefault(); setIsDragging(true); }, []);
  const handleDragLeave = useCallback((e: React.DragEvent) => { e.preventDefault(); setIsDragging(false); }, []);
  const handleDrop = useCallback((e: React.DragEvent) => {
    e.preventDefault();
    setIsDragging(false);
    const file = e.dataTransfer.files[0];
    if (file) onFileSelect(file);
  }, [onFileSelect]);

  if (apk) {
    return (
      <Card className={cn("relative overflow-hidden", isMobile ? "border-0 shadow-none bg-muted/50" : "")}>
        <CardContent className={cn("p-4", isMobile && "px-0")}>
          <div className="flex items-center gap-4">
            {apk.iconUrl ? (
              <img src={apk.iconUrl} alt="App Icon" className="h-16 w-16 rounded-xl object-cover shadow-lg border border-border" />
            ) : (
              <div className="flex h-16 w-16 items-center justify-center rounded-xl bg-muted"><FileArchive className="h-8 w-8 text-muted-foreground" /></div>
            )}
            <div className="flex-1 min-w-0">
              <h3 className="font-semibold truncate">{apk.file.name}</h3>
              <p className="text-sm text-muted-foreground">{formatFileSize(apk.file.size)}</p>
              {apk.info?.success && <p className="text-xs text-muted-foreground mt-1">{apk.info.package_name} • v{apk.info.version_name}</p>}
            </div>
            <Button variant="ghost" size="icon" onClick={onClear} className="shrink-0"><X className="h-4 w-4" /></Button>
          </div>
        </CardContent>
      </Card>
    );
  }

  return (
    <>
      <input ref={fileInputRef} type="file" accept=".apk" onChange={(e) => e.target.files?.[0] && onFileSelect(e.target.files[0])} className="hidden" />
      <Card
        className={cn("cursor-pointer transition-all duration-200 border-dashed border-2", isDragging && "border-primary bg-primary/5", isMobile ? "border-0 shadow-none bg-muted/30" : "hover:border-primary/50 hover:bg-muted/30")}
        onClick={() => fileInputRef.current?.click()}
        onDragOver={handleDragOver}
        onDragLeave={handleDragLeave}
        onDrop={handleDrop}
      >
        <CardContent className={cn("flex flex-col items-center justify-center gap-4 text-center", isMobile ? "py-8" : "py-12")}>
          {isLoading ? (
            <Loader2 className="h-12 w-12 text-primary animate-spin" />
          ) : (
            <div className="relative">
              <div className="absolute inset-0 bg-primary/20 blur-xl rounded-full" />
              <Upload className="relative h-12 w-12 text-primary" />
            </div>
          )}
          <div>
            <p className="font-medium">{isLoading ? 'Loading APK...' : 'Drop your APK file here'}</p>
            <p className="text-sm text-muted-foreground mt-1">{isLoading ? 'Please wait' : 'or click to browse'}</p>
          </div>
        </CardContent>
      </Card>
    </>
  );
}

// ============ MANIFEST EDITOR ============
function ManifestEditor({ values, onChange, originalInfo, wasmModule, isMobile = false }: {
  values: ManifestValues;
  onChange: (values: ManifestValues) => void;
  originalInfo: ApkInfo | null;
  wasmModule: WasmModule | null;
  isMobile?: boolean;
}) {
  const isPackageValid = !values.packageName || (wasmModule?.validate_package_name(values.packageName) ?? true);
  const handleChange = (field: keyof ManifestValues, value: string) => onChange({ ...values, [field]: value });

  const fields = [
    { id: 'packageName', label: 'Package Name', icon: Package, placeholder: 'com.example.myapp', hint: 'e.g., com.example.myapp', value: values.packageName, isValid: isPackageValid, showValidation: !!values.packageName },
    { id: 'appName', label: 'App Name', icon: Type, placeholder: originalInfo?.app_name === '(resource reference)' ? '(uses resource reference)' : 'My Application', hint: 'Display name in launcher', value: values.appName, isValid: true, showValidation: false },
    { id: 'versionCode', label: 'Version Code', icon: Hash, placeholder: '1', hint: 'Integer value', value: values.versionCode, type: 'number', isValid: true, showValidation: false },
    { id: 'versionName', label: 'Version Name', icon: Tag, placeholder: '1.0.0', hint: 'e.g., 1.0.0', value: values.versionName, isValid: true, showValidation: false },
  ];

  if (isMobile) {
    return (
      <div className="space-y-4">
        {fields.map((field) => (
          <div key={field.id} className="space-y-2">
            <div className="flex items-center justify-between">
              <Label htmlFor={field.id} className="flex items-center gap-2 text-sm"><field.icon className="h-4 w-4 text-muted-foreground" />{field.label}</Label>
              {field.showValidation && <Badge variant={field.isValid ? 'secondary' : 'destructive'} className="text-xs">{field.isValid ? <Check className="h-3 w-3" /> : <X className="h-3 w-3" />}</Badge>}
            </div>
            <Input id={field.id} type={field.type || 'text'} value={field.value} onChange={(e) => handleChange(field.id as keyof ManifestValues, e.target.value)} placeholder={field.placeholder} className={cn("h-11", field.showValidation && !field.isValid && "border-destructive")} />
            <p className="text-xs text-muted-foreground">{field.hint}</p>
          </div>
        ))}
      </div>
    );
  }

  return (
    <Card>
      <CardHeader><CardTitle className="flex items-center gap-2 text-lg"><Pencil className="h-5 w-5 text-primary" />Edit Manifest Properties</CardTitle></CardHeader>
      <CardContent className="space-y-6">
        <div className="grid gap-6 md:grid-cols-2">
          {fields.slice(0, 2).map((field) => (
            <div key={field.id} className="space-y-2">
              <div className="flex items-center justify-between">
                <Label htmlFor={field.id} className="flex items-center gap-2"><field.icon className="h-4 w-4 text-muted-foreground" />{field.label}</Label>
                {field.showValidation && <Badge variant={field.isValid ? 'secondary' : 'destructive'} className="text-xs">{field.isValid ? <><Check className="h-3 w-3 mr-1" /> Valid</> : <><X className="h-3 w-3 mr-1" /> Invalid</>}</Badge>}
              </div>
              <Input id={field.id} value={field.value} onChange={(e) => handleChange(field.id as keyof ManifestValues, e.target.value)} placeholder={field.placeholder} className={cn(field.showValidation && !field.isValid && "border-destructive")} />
              <p className="text-xs text-muted-foreground">{field.hint}</p>
            </div>
          ))}
        </div>
        <div className="grid gap-6 md:grid-cols-3">
          {fields.slice(2).map((field) => (
            <div key={field.id} className="space-y-2">
              <Label htmlFor={field.id} className="flex items-center gap-2"><field.icon className="h-4 w-4 text-muted-foreground" />{field.label}</Label>
              <Input id={field.id} type={field.type || 'text'} value={field.value} onChange={(e) => handleChange(field.id as keyof ManifestValues, e.target.value)} placeholder={field.placeholder} />
              <p className="text-xs text-muted-foreground">{field.hint}</p>
            </div>
          ))}
        </div>
        <div className="flex flex-wrap gap-2 pt-2">
          {['Edit Package Name', 'Edit App Name', 'Edit Version', 'Auto Sign APK'].map((feature) => (
            <Badge key={feature} variant="outline" className="gap-1"><Check className="h-3 w-3 text-green-500" />{feature}</Badge>
          ))}
        </div>
      </CardContent>
    </Card>
  );
}

// ============ SIGNING OPTIONS ============
function SigningOptions({ config, onChange, wasmModule, isMobile = false }: {
  config: SigningConfig;
  onChange: (config: SigningConfig) => void;
  wasmModule: WasmModule | null;
  isMobile?: boolean;
}) {
  const [passwordValid, setPasswordValid] = useState<boolean | null>(null);
  const [isValidating, setIsValidating] = useState(false);
  const fileInputRef = useRef<HTMLInputElement>(null);

  const handleKeystoreFile = async (e: React.ChangeEvent<HTMLInputElement>) => {
    const file = e.target.files?.[0];
    if (!file) return;
    const buffer = await file.arrayBuffer();
    onChange({
      ...config,
      keystoreData: new Uint8Array(buffer),
      keystoreFileName: file.name,
      aliases: [],
      selectedAlias: '',
    });
    setPasswordValid(null);
  };

  // Validate store password and fetch aliases when typing (debounced)
  useEffect(() => {
    if (!wasmModule || !config.keystoreData || !config.keystorePassword) {
      setPasswordValid(null);
      if (config.aliases.length > 0) {
        onChange({ ...config, aliases: [], selectedAlias: '' });
      }
      return;
    }
    setIsValidating(true);
    const timeoutId = setTimeout(() => {
      try {
        const isValid = wasmModule.verify_keystore_password(config.keystoreData!, config.keystorePassword);
        setPasswordValid(isValid);
        if (isValid) {
          // Fetch aliases
          const aliases = wasmModule.get_keystore_aliases(config.keystoreData!, config.keystorePassword);
          const firstAlias = aliases.length > 0 ? aliases[0] : '';
          onChange({
            ...config,
            aliases,
            selectedAlias: firstAlias,
          });
        } else {
          onChange({ ...config, aliases: [], selectedAlias: '' });
        }
      } catch {
        setPasswordValid(false);
        onChange({ ...config, aliases: [], selectedAlias: '' });
      }
      setIsValidating(false);
    }, 300);
    return () => clearTimeout(timeoutId);
  }, [config.keystoreData, config.keystorePassword, wasmModule]);

  const handleToggleCustomKey = (checked: boolean | "indeterminate") => {
    if (checked === "indeterminate") return;
    onChange({
      ...config,
      useCustomKey: checked,
      keystoreData: null,
      keystorePassword: '',
      keystoreFileName: '',
      aliases: [],
      selectedAlias: '',
    });
    setPasswordValid(null);
  };

  if (isMobile) {
    return (
      <div className="space-y-4">
        <label className="flex items-center gap-3 p-3 rounded-lg border border-border bg-muted/30 cursor-pointer">
          <Checkbox
            checked={config.useCustomKey}
            onCheckedChange={handleToggleCustomKey}
          />
          <div className="flex-1">
            <div className="flex items-center gap-2">
              <Key className="h-4 w-4 text-muted-foreground" />
              <span className="font-medium">Use Custom Keystore</span>
            </div>
            <p className="text-xs text-muted-foreground mt-0.5">Sign with your own .keystore, .jks, or .p12 file</p>
          </div>
        </label>

        {config.useCustomKey && (
          <div className="space-y-3 pl-3 border-l-2 border-primary animate-in slide-in-from-top-2">
            <div className="space-y-2">
              <Label>Keystore File</Label>
              <input ref={fileInputRef} type="file" accept=".keystore,.jks,.p12,.pfx" onChange={handleKeystoreFile} className="hidden" />
              <Button variant="outline" className="w-full h-11 justify-start" onClick={() => fileInputRef.current?.click()}>
                <Key className="h-4 w-4 mr-2" />
                {config.keystoreFileName || 'Select keystore file...'}
              </Button>
              <p className="text-xs text-muted-foreground">Supports .keystore, .jks, .p12, .pfx</p>
            </div>
            <div className="space-y-2">
              <div className="flex items-center justify-between">
                <Label>Store Password</Label>
                {config.keystoreData && config.keystorePassword && (
                  <Badge variant={passwordValid === true ? 'secondary' : passwordValid === false ? 'destructive' : 'outline'} className="text-xs">
                    {isValidating ? <Loader2 className="h-3 w-3 animate-spin" /> : passwordValid ? <Check className="h-3 w-3" /> : passwordValid === false ? <X className="h-3 w-3" /> : null}
                  </Badge>
                )}
              </div>
              <Input
                type="password"
                value={config.keystorePassword}
                onChange={(e) => onChange({ ...config, keystorePassword: e.target.value })}
                placeholder="Enter keystore password"
                className="h-11"
                disabled={!config.keystoreData}
              />
            </div>
            {passwordValid && config.aliases.length > 0 && (
              <div className="space-y-2">
                <Label>Key Alias</Label>
                <select
                  value={config.selectedAlias}
                  onChange={(e) => onChange({ ...config, selectedAlias: e.target.value })}
                  className="w-full h-11 px-3 rounded-md border border-input bg-background text-sm"
                >
                  {config.aliases.map((alias) => (
                    <option key={alias} value={alias}>{alias}</option>
                  ))}
                </select>
              </div>
            )}
          </div>
        )}
      </div>
    );
  }

  return (
    <Card>
      <CardHeader><CardTitle className="flex items-center gap-2 text-lg"><Lock className="h-5 w-5 text-primary" />Signing Options</CardTitle></CardHeader>
      <CardContent className="space-y-4">
        <label className="flex items-center gap-4 p-4 rounded-lg border-2 border-border hover:border-primary/50 cursor-pointer transition-all">
          <Checkbox
            checked={config.useCustomKey}
            onCheckedChange={handleToggleCustomKey}
          />
          <div className="flex-1">
            <div className="flex items-center gap-2">
              <Key className="h-5 w-5 text-muted-foreground" />
              <span className="font-semibold">Use Custom Keystore</span>
            </div>
            <p className="text-sm text-muted-foreground mt-1">Sign with your own .keystore, .jks, or .p12 file instead of the debug key</p>
          </div>
        </label>

        {config.useCustomKey && (
          <div className="space-y-4 pl-4 border-l-2 border-primary animate-in slide-in-from-top-2">
            <div className="grid gap-4 md:grid-cols-2">
              <div className="space-y-2">
                <Label>Keystore File</Label>
                <input ref={fileInputRef} type="file" accept=".keystore,.jks,.p12,.pfx" onChange={handleKeystoreFile} className="hidden" />
                <Button variant="outline" className="w-full justify-start" onClick={() => fileInputRef.current?.click()}>
                  <Key className="h-4 w-4 mr-2" />
                  {config.keystoreFileName || 'Select keystore file...'}
                </Button>
                <p className="text-xs text-muted-foreground">Supports .keystore, .jks, .p12, .pfx formats</p>
              </div>
              <div className="space-y-2">
                <div className="flex items-center justify-between">
                  <Label>Store Password</Label>
                  {config.keystoreData && config.keystorePassword && (
                    <Badge variant={passwordValid === true ? 'secondary' : passwordValid === false ? 'destructive' : 'outline'}>
                      {isValidating ? (
                        <><Loader2 className="h-3 w-3 mr-1 animate-spin" /> Checking...</>
                      ) : passwordValid ? (
                        <><Check className="h-3 w-3 mr-1" /> Valid</>
                      ) : passwordValid === false ? (
                        <><X className="h-3 w-3 mr-1" /> Invalid</>
                      ) : null}
                    </Badge>
                  )}
                </div>
                <Input
                  type="password"
                  value={config.keystorePassword}
                  onChange={(e) => onChange({ ...config, keystorePassword: e.target.value })}
                  placeholder="Enter keystore password"
                  disabled={!config.keystoreData}
                />
              </div>
            </div>
            {passwordValid && config.aliases.length > 0 && (
              <div className="space-y-2 animate-in slide-in-from-top-2">
                <Label>Key Alias</Label>
                <select
                  value={config.selectedAlias}
                  onChange={(e) => onChange({ ...config, selectedAlias: e.target.value })}
                  className="w-full h-10 px-3 rounded-md border border-input bg-background text-sm focus:outline-none focus:ring-2 focus:ring-ring"
                >
                  {config.aliases.map((alias) => (
                    <option key={alias} value={alias}>{alias}</option>
                  ))}
                </select>
                <p className="text-xs text-muted-foreground">Select the key to use for signing</p>
              </div>
            )}
          </div>
        )}

        {!config.useCustomKey && (
          <div className="flex items-center gap-2 p-3 bg-muted/50 rounded-lg text-sm text-muted-foreground">
            <Shield className="h-4 w-4 text-green-500" />
            <span>APK will be signed with the default debug key (APK Signature Scheme v2)</span>
          </div>
        )}
      </CardContent>
    </Card>
  );
}

// ============ LOADING SCREEN ============
function LoadingScreen({ progress }: { progress: number }) {
  return (
    <div className="fixed inset-0 z-50 flex flex-col items-center justify-center bg-background">
      <div className="flex flex-col items-center gap-6">
        <div className="relative">
          <div className="absolute inset-0 bg-primary/20 blur-2xl rounded-full animate-pulse" />
          <Smartphone className="relative h-16 w-16 text-primary loading-pulse" />
        </div>
        <div className="text-center space-y-2">
          <h2 className="text-xl font-semibold">Loading APK Editor</h2>
          <p className="text-sm text-muted-foreground">Initializing WASM module...</p>
        </div>
        <div className="w-48"><Progress value={progress} className="h-1" /></div>
      </div>
    </div>
  );
}

// ============ ERROR SCREEN ============
function ErrorScreen({ error }: { error: string }) {
  return (
    <div className="fixed inset-0 z-50 flex flex-col items-center justify-center bg-background p-4">
      <div className="text-center space-y-4 max-w-md">
        <div className="mx-auto h-16 w-16 rounded-full bg-destructive/10 flex items-center justify-center"><Smartphone className="h-8 w-8 text-destructive" /></div>
        <h2 className="text-xl font-semibold">Failed to Load</h2>
        <p className="text-sm text-muted-foreground">{error}</p>
        <Button onClick={() => window.location.reload()}>Retry</Button>
      </div>
    </div>
  );
}

// ============ MAIN APK EDITOR ============
function ApkEditorCore({ wasmModule, apk, isLoadingApk, apkError, onLoadApk, onClearApk, isMobile = false }: {
  wasmModule: WasmModule | null;
  apk: ApkFile | null;
  isLoadingApk: boolean;
  apkError: string | null;
  onLoadApk: (file: File) => void;
  onClearApk: () => void;
  isMobile?: boolean;
}) {
  const [isProcessing, setIsProcessing] = useState(false);
  const [progress, setProgress] = useState(0);
  const [manifestValues, setManifestValues] = useState<ManifestValues>({ packageName: '', appName: '', versionCode: '', versionName: '' });
  const [signingConfig, setSigningConfig] = useState<SigningConfig>({ useCustomKey: false, keystoreData: null, keystorePassword: '', keystoreFileName: '', aliases: [], selectedAlias: '' });

  useEffect(() => {
    if (apk?.info?.success) {
      setManifestValues({
        packageName: apk.info.package_name || '',
        appName: apk.info.app_name === '(resource reference)' ? '' : (apk.info.app_name || ''),
        versionCode: apk.info.version_code > 0 ? String(apk.info.version_code) : '',
        versionName: apk.info.version_name || '',
      });
    }
  }, [apk]);

  const handleProcess = async () => {
    if (!wasmModule || !apk) return;

    const packageName = manifestValues.packageName.trim() || null;
    const appName = manifestValues.appName.trim() || null;
    const versionCode = manifestValues.versionCode.trim() ? parseInt(manifestValues.versionCode, 10) : null;
    const versionName = manifestValues.versionName.trim() || null;

    if (!packageName && !appName && !versionCode && !versionName) { toast.error('Please enter at least one property to edit'); return; }
    if (packageName && !wasmModule.validate_package_name(packageName)) { toast.error('Invalid package name format'); return; }
    if (signingConfig.useCustomKey) {
      if (!signingConfig.keystoreData) { toast.error('Please upload a keystore file'); return; }
      if (!wasmModule.verify_keystore_password(signingConfig.keystoreData, signingConfig.keystorePassword)) { toast.error('Invalid store password'); return; }
      if (!signingConfig.selectedAlias) { toast.error('No key alias selected'); return; }
    }

    setIsProcessing(true);
    setProgress(10);

    try {
      setProgress(30);
      let result;
      if (signingConfig.useCustomKey && signingConfig.keystoreData) {
        result = wasmModule.edit_apk_with_keystore(
          apk.data,
          packageName,
          appName,
          versionCode,
          versionName,
          signingConfig.keystoreData,
          signingConfig.keystorePassword,
          signingConfig.selectedAlias || null,
          signingConfig.keystorePassword // Key password is same as store password
        );
      } else {
        result = wasmModule.edit_apk(apk.data, packageName, appName, versionCode, versionName);
      }

      setProgress(80);

      if (result.success) {
        const modifiedData = result.get_data();
        const blob = new Blob([modifiedData], { type: 'application/vnd.android.package-archive' });
        const url = URL.createObjectURL(blob);
        const a = document.createElement('a');
        a.href = url;
        a.download = apk.file.name.replace('.apk', '_edited.apk');
        document.body.appendChild(a);
        a.click();
        document.body.removeChild(a);
        URL.revokeObjectURL(url);
        setProgress(100);
        toast.success('APK edited successfully!', { description: `Signed with: ${signingConfig.useCustomKey ? `Custom Keystore (${signingConfig.selectedAlias})` : 'Debug Key'}` });
      } else {
        toast.error('Error processing APK', { description: result.error_message });
      }
    } catch (error) {
      toast.error('Error processing APK', { description: error instanceof Error ? error.message : 'Unknown error' });
    } finally {
      setIsProcessing(false);
      setProgress(0);
    }
  };

  const canProcess = apk && !isProcessing;

  if (isMobile) {
    return (
      <div className="flex flex-col gap-6 pb-32">
        {apkError && <Alert variant="destructive"><AlertCircle className="h-4 w-4" /><AlertTitle>Error</AlertTitle><AlertDescription>{apkError}</AlertDescription></Alert>}
        <section><h2 className="text-sm font-semibold text-muted-foreground mb-3 uppercase tracking-wide">Select APK</h2><FileDropzone onFileSelect={onLoadApk} apk={apk} isLoading={isLoadingApk} onClear={onClearApk} isMobile /></section>
        {apk && (
          <>
            <section><h2 className="text-sm font-semibold text-muted-foreground mb-3 uppercase tracking-wide">Edit Manifest</h2><ManifestEditor values={manifestValues} onChange={setManifestValues} originalInfo={apk.info} wasmModule={wasmModule} isMobile /></section>
            <section><h2 className="text-sm font-semibold text-muted-foreground mb-3 uppercase tracking-wide">Signing</h2><SigningOptions config={signingConfig} onChange={setSigningConfig} wasmModule={wasmModule} isMobile /></section>
          </>
        )}
        <div className="fixed bottom-0 left-0 right-0 p-4 pb-8 bg-background/95 backdrop-blur border-t safe-bottom">
          {isProcessing && <Progress value={progress} className="mb-3 h-1" />}
          <Button onClick={handleProcess} disabled={!canProcess} className="w-full h-12 text-base font-semibold" size="lg">
            {isProcessing ? <><Loader2 className="mr-2 h-5 w-5 animate-spin" />Processing...</> : <><Sparkles className="mr-2 h-5 w-5" />Edit & Sign APK</>}
          </Button>
        </div>
      </div>
    );
  }

  return (
    <div className="space-y-6">
      {apkError && <Alert variant="destructive"><AlertCircle className="h-4 w-4" /><AlertTitle>Error</AlertTitle><AlertDescription>{apkError}</AlertDescription></Alert>}
      <FileDropzone onFileSelect={onLoadApk} apk={apk} isLoading={isLoadingApk} onClear={onClearApk} />
      {apk && (
        <>
          <ManifestEditor values={manifestValues} onChange={setManifestValues} originalInfo={apk.info} wasmModule={wasmModule} />
          <SigningOptions config={signingConfig} onChange={setSigningConfig} wasmModule={wasmModule} />
          <div className="space-y-4">
            {isProcessing && <Progress value={progress} className="h-2" />}
            <Button onClick={handleProcess} disabled={!canProcess} className="w-full h-12 text-base font-semibold" size="lg">
              {isProcessing ? <><Loader2 className="mr-2 h-5 w-5 animate-spin" />Processing APK...</> : <><Sparkles className="mr-2 h-5 w-5" />Edit & Sign APK</>}
            </Button>
          </div>
        </>
      )}
    </div>
  );
}

// ============ MAIN APP ============
export default function App() {
  const device = useDevice();
  const { isLoading: isWasmLoading, isReady, error: wasmError, version, module } = useWasm();
  const { apk, isLoading: isLoadingApk, error: apkError, loadApk, clearApk } = useApk(module);
  const [loadProgress, setLoadProgress] = useState(0);

  useEffect(() => {
    if (isWasmLoading) {
      const interval = setInterval(() => setLoadProgress((prev) => Math.min(prev + 10, 90)), 200);
      return () => clearInterval(interval);
    } else if (isReady) {
      setLoadProgress(100);
    }
  }, [isWasmLoading, isReady]);

  if (isWasmLoading) return <LoadingScreen progress={loadProgress} />;
  if (wasmError) return <ErrorScreen error={wasmError} />;

  if (device.isMobile) {
    return (
      <div className="flex min-h-screen flex-col">
        <Header version={version} isMobile />
        <main className="flex-1 px-4 py-4"><ApkEditorCore wasmModule={module} apk={apk} isLoadingApk={isLoadingApk} apkError={apkError} onLoadApk={loadApk} onClearApk={clearApk} isMobile /></main>
        <Footer isMobile />
      </div>
    );
  }

  return (
    <div className="flex min-h-screen flex-col">
      <Header version={version} />
      <main className="flex-1 container mx-auto px-6 py-8 max-w-4xl"><ApkEditorCore wasmModule={module} apk={apk} isLoadingApk={isLoadingApk} apkError={apkError} onLoadApk={loadApk} onClearApk={clearApk} /></main>
      <Footer />
    </div>
  );
}
