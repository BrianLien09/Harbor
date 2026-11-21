<div align="center">

<img width="50%" alt="Harbor" src="https://github.com/user-attachments/assets/8c715a53-a0df-4780-9dd4-09ce3ccd9043" />

  **為 Windows 10 與 11 打造的輕量級 macOS 風格 Dock。**

  Harbor 以可自訂的 Dock 取代 Windows 工作列，並提供選用的 macOS 風格選單列。
</div>

<img width="1500" height="436" alt="Harbor 的 Dock 與選單列" src="https://github.com/user-attachments/assets/3c91b488-214d-453d-a8c2-a9e0dc7f245e" />

<img width="1000" height="259" alt="Dock 圖示放大效果" src="https://github.com/user-attachments/assets/e5b2ce63-cec1-4d1d-8ef1-a4fa1c668c40" />


圖示平順放大、即時視窗預覽，以及選用的神燈（Genie）最小化效果。

| Clawd 吉祥物 | 最小化神燈效果 |
| :---: | :---: |
| ![吉祥物出現在 Dock 與選單列上方的 GIF](https://github.com/user-attachments/assets/580b235e-6141-493a-a333-8f2b96f3acca) | ![視窗以神燈效果最小化進 Dock 的 GIF](https://github.com/user-attachments/assets/878503a8-7286-445d-826b-41c18da013bc) |
| **用量預覽** | **用量詳細資訊** |
| ![Dock 與選單列自動隱藏再重新出現的 GIF](https://github.com/user-attachments/assets/60a8f7c0-20fc-429b-977f-d5b4b7d97d88) | ![視窗以神燈效果最小化進 Dock 的 GIF](https://github.com/user-attachments/assets/fbf21cf6-c1c0-4173-9d68-ae883f0d2a45) |

## 功能

- 毛玻璃風格的 Dock，圖示會依游標位置平順放大。
- 啟動、釘選、重新排序與管理應用程式，包含最近使用的應用程式與已開啟的視窗。
- 將釘選的應用程式群組成單一 Dock 圖示：把一個應用程式拖放到另一個上方，點擊群組即可展開，
  再從展開的項目中拖出來就能取消群組。
- 游標停在有多個視窗的應用程式上，會顯示附標題的即時預覽；點擊預覽即可切換到該視窗。
  在預覽上滾動可查看更多視窗，按 Esc 則關閉預覽。
- 最小化的視窗會以神燈效果收進所屬應用程式的圖示；若開啟「設定 → 視窗 → *顯示最小化的視窗*」，
  則會改以即時縮圖的形式保留在 Dock 右側。
- 點擊已在最前方的應用程式即可將它最小化，與 Windows 工作列的行為相同。
- 可將檔案拖放到應用程式、「下載」或資源回收筒上。
- Dock 右側有捷徑資料夾：內容會以選單展開，資料夾內的每個子資料夾也有各自的子選單，
  類似 Windows 10 工作列的工具列。
- 選用的自動隱藏。
- 頂端選單列會顯示目前最前方的應用程式、時鐘、網路、音量與電池、輸入法、使用中的麥克風／相機、
  隱藏的系統匣圖示，以及（若你的 Windows 工作列有開啟）搜尋、工作檢視與小工具按鈕。
- 選單列上有一座弧形的「島」，顯示 Claude Code、Codex、OpenCode Go、Gemini CLI 與 Antigravity
  的用量上限。Gemini Pro 與 Flash 的額度在可取得時會分開顯示。
- 獨立的設定應用程式，變更即時生效，支援西班牙文、英文、法文與繁體中文，
  並可設定神燈最小化動畫的行為。
- 若 Dock 意外結束，守護程式會自動還原 Windows 工作列。

神燈效果**預設為開啟**，可在「設定 → 視窗」中關閉。啟用期間，Harbor 會暫時停用被追蹤視窗的
Windows 視窗轉場動畫，好讓自己的最小化動畫能夠播放；Harbor 結束時，這些轉場動畫會自動還原。

## 安裝

**[下載 Harbor-Setup.exe](https://github.com/BrianLien09/Harbor/releases/latest/download/Harbor-Setup.exe)**
（Windows 10 1903 版或更新版本，或 Windows 11；64 位元），也可以從
[Releases](https://github.com/BrianLien09/Harbor/releases) 選擇其他版本。

1. 執行 `Harbor-Setup.exe`。它只為目前的使用者安裝：不需要系統管理員權限、沒有安裝精靈，
   大約兩秒就完成。
2. Harbor 還很新，Windows SmartScreen 可能尚未認得它，並顯示已保護你的電腦。
   請選擇 **其他資訊 → 仍要執行**。
3. 從開始功能表開啟 **Harbor**，再選擇 **Dock** 來自訂 Dock 與頂端選單列。

## 在 Windows 上試用

### 事前準備

- Windows 10（1903 版或更新版本）或 Windows 11。在 Windows 10 上，Dock 與選單列是實色而非毛玻璃：
  Windows 10 沒有提供桌面應用程式使用的背景模糊效果。
- 安裝 Rust（MSVC 工具鏈）與 Visual Studio C++ Build Tools
- 安裝 Node.js 與 pnpm

### 執行 Dock

在儲存庫根目錄執行：

```powershell
cargo run -p harbor
```

Dock 執行期間會隱藏工作列。在終端機按 **Ctrl+C** 即可結束；Harbor 應該會自動還原工作列。
若需要手動還原：

```powershell
cargo run -p harbor -- --restore
```

### 執行設定應用程式

保持 Dock 執行，開啟第二個終端機，然後執行：

```powershell
cd apps\settings
pnpm install
pnpm tauri dev
```

這會開啟原生的設定視窗。前端的變更會熱重載；修改 Rust 後端之後，請重新啟動 `tauri dev`。
設定的變更會即時套用到執行中的 Dock。

## 建置與測試

在儲存庫根目錄執行：

```powershell
cargo fmt --all -- --check
cargo test --workspace --locked
```

建置設定應用程式的前端：

```powershell
cd apps\settings
pnpm build
```

## 專案結構

| 路徑 | 用途 |
|---|---|
| `apps/dock` | 原生 Windows Dock |
| `apps/settings` | Tauri 桌面設定應用程式 |
| `crates/harbor-core` | 與平台無關的版面配置與動畫邏輯 |
| `crates/harbor-shell` | Windows Shell 整合 |
| `crates/harbor-config` | 共用設定與持久化 |
| `site` | Astro 網站 |

## 隱私權

Harbor 不收集任何資料、沒有遙測，也絕不會把任何內容傳送給專案團隊。它會發出兩種網路請求，
兩者皆為選用：

- **更新：** 設定應用程式開啟時，會向 GitHub Releases 詢問是否有較新的版本；此外每天一次，
  在開機後幾分鐘，Harbor 會在背景做同樣的檢查，並安裝找到的更新（Dock 本身從不使用網路）。
  可在「設定 → 系統 → *自動安裝更新*」關閉安裝，或用 *自動檢查更新* 關閉全部。
- **AI 供應商用量：** 當支援的用戶端正在執行時，Harbor 會讀取其現有的本機登入狀態，
  並在顯示期間每五分鐘、或按下重新整理時，向該供應商查詢可用的方案上限。
  對於 Gemini CLI 與 Antigravity，Gemini Pro 與 Flash 的額度在可取得時會分開顯示；
  Antigravity 是透過其本機服務查詢。認證資訊只保留在記憶體中，Harbor 不會儲存。
  可在「設定 → 選單列 → *顯示 AI 供應商用量*」關閉。

詳細說明：[brianlien09.github.io/Harbor/privacy](https://brianlien09.github.io/Harbor/privacy)。

## 致謝與授權

Harbor 以 [Muelle](https://github.com/crafter-station/Muelle)（Crafter Station 的 MIT 授權專案）為基礎，
在此之上持續開發，並加入繁體中文介面等功能。感謝原作者開源這個專案。

Harbor 以 [MIT 授權](LICENSE) 發布，原專案的版權聲明保留在 LICENSE 中；
所用到的第三方授權聲明請見 [THIRD-PARTY-NOTICES.md](THIRD-PARTY-NOTICES.md)。
