# 更新紀錄

從 0.1.18 起，每個版本的更新內容以繁體中文撰寫。發布流程會把該版本的段落
（`## [版本]` 到下一個 `## [` 之間）原樣複製成 GitHub Release 的說明。
0.1.17 以前的段落沿用原專案的英文與西班牙文。

## [Unreleased]

## [0.1.18]

### Harbor 專屬

- 專案改名為 Harbor，改用自己的 GitHub 倉庫發布；app 內自動更新也改從這裡下載新版。
- 新增繁體中文介面，Dock、選單列與設定都有中文；README 預設改為繁體中文。
- 頂部選單列可顯示系統匣（通知區域）的釘選圖示，再按一次圖示即可收起選單。
- 神燈最小化不再先閃一下桌面、也不會停頓才起飛：替身畫面事先備好，按下最小化鈕
  時就擺到視窗後方，起飛只等擷取畫面（約 0.1 秒）。從 Dock 還原不會再冒出 Windows
  原生動畫，同時最小化的視窗最多 8 個都保有神燈還原效果。

### 新功能

- 設定 → Dock → 螢幕位置：可選左側、下方或右側（預設下方）。放在側邊時 Dock 由上往下
  排列，選單、群組與視窗預覽會在圖示旁打開，神燈也會往側邊飛。
- 設定 → 視窗 → 神燈速度：一般（0.5 秒）、快（0.35 秒）、更快（0.25 秒）。
- 執行中 app 的右鍵選單新增「結束工作」，群組裡的 app 也有；會先確認選取的程序，
  不會誤關 Explorer 等系統程序。
- 需要注意的 app（Windows 要求注意時）會在圖示上顯示紅色提示點，切到該視窗或關閉後消失。
- 設定 → 選單列 → 顯示正在播放：可以關掉島狀區塊的音樂部分。
- 音樂島與彈出視窗顯示圓形專輯封面，彈出視窗新增系統音量與靜音控制。
- 選單列新增精簡的泡泡樣式（音樂與 AI 用量）、省電模式電池提示、Wi-Fi 警示與藍牙小工具。
- 設定頁加上 Dock 位置、島狀樣式與神燈速度的示意圖，預覽會使用目前螢幕的桌布。
- OpenCode 用量卡片分開顯示 Go 額度與 Zen 近 30 天預估花費。

### 修正

- 已解除安裝的 app 不會再卡在「最近使用」；最近使用的 app 右鍵也能「從 Dock 移除」。
- 接收器連線的 Logitech 滑鼠（例如 G305）休眠時，選單列仍會保留最後的電量。
- 修正 OpenCode Go 帳號偵測失敗。
- 安裝某些第三方檔案總管擴充（例如 Uninstall Tool）時，開 app 不會再跳出
  「Ordinal 381 Not Found」錯誤視窗。
- 多螢幕時工作列隱藏更穩定，副螢幕的工作列不會閃出來。

## [0.1.17]

### English

- Added Google Search Console ownership verification to the website.
- Automated WinGet manifest generation and submission for the first package and
  future stable releases, with manual retries, native validation, final installer
  checksum checks, duplicate detection and explicit missing-token failures.
- Added bounded graphics-device recovery and recovery of an existing dock when
  Muelle is opened again. The watchdog closes the old instance before restarting;
  a blocked recovery close has an eight-second deadline. Repeated device loss
  within the first minute after recovery stops automatic restarts and restores
  the Windows taskbar. Graphics health reuses the existing watchdog heartbeat.
- An error updating the top bar no longer prevents the dock from refreshing or
  returning after full-screen mode.
- Added read-only USB adapters for SteelSeries Nova 5/5X and more Nova 7/7P
  variants, Logitech G PRO X 2 (Centurion), selected G PRO/G533 HID++ variants,
  HyperX Cloud Alpha Wireless and Kingston Cloud II Wireless. Voltage-only and
  discrete readings remain unknown; HP/DTS Cloud II revisions with a reported
  connection-query side effect are excluded. Physical validation is pending.
- Added Logitech Lightspeed and selected Nano receiver IDs, including the G305's
  C53F, and HID++ Battery Status 1000 reads. Short and long report collections are
  now queried together once per receiver. The 60-second interval and opt-in
  setting remain; physical validation of the expanded coverage is pending.
- Fixed three Clippy violations in the accessory diagnostic's serialization tests.
  Tag releases now run the full CI checks before building and signing the installer.

### Español

- Añadida la verificación de propiedad de Google Search Console en la web.
- Automatizada la generación y el envío de manifiestos de WinGet para la primera
  publicación y las próximas releases estables, con reintentos manuales, validación
  nativa, comprobación del hash final, detección de duplicados y error si falta el token.
- Añadida recuperación limitada ante pérdida del dispositivo gráfico y al abrir
  Muelle de nuevo cuando ya existe una instancia. El guardián cierra la anterior
  antes de reiniciarla; si el cierre de recuperación se bloquea, tiene un plazo de
  ocho segundos. Un nuevo fallo gráfico durante el primer minuto tras recuperar
  detiene los reinicios automáticos y devuelve la barra de Windows. La revisión
  gráfica aprovecha la comprobación de salud que ya hacía el guardián.
- Un error al actualizar la barra superior ya no impide actualizar el dock ni
  hacerlo reaparecer al salir de pantalla completa.
- Añadidos adaptadores USB de lectura para SteelSeries Nova 5/5X y más variantes
  Nova 7/7P, Logitech G PRO X 2 (Centurion), variantes G PRO/G533 HID++, HyperX
  Cloud Alpha Wireless y Cloud II Wireless Kingston. Voltajes y niveles discretos
  siguen sin porcentaje; se excluyen revisiones HP/DTS de Cloud II cuya consulta
  de conexión tiene un efecto secundario documentado. Falta validación física.
- Añadidos receptores Logitech Lightspeed y Nano identificados, incluido C53F del
  G305, y lectura de batería HID++ 1000. Las colecciones de informes cortos y largos
  se consultan juntas una sola vez por receptor. Se mantienen los 60 segundos y
  la activación opcional; la ampliación queda pendiente de validación física.
- Corregidos tres errores de Clippy en las pruebas de serialización del diagnóstico
  de accesorios. Las releases por etiqueta ejecutan ahora todos los controles de CI
  antes de compilar y firmar el instalador.

## [0.1.16]

### English

- Initial automatic USB battery adapters for allowlisted Logitech Unifying/Bolt
  receivers and SteelSeries Nova 7 variants. Queries run in disposable processes,
  pause with the hidden menu bar and use a 60-second interval with error backoff.
  Approximate levels stay unknown. Physical validation is still pending; radio
  link changes may take until the next query to appear.
- Optional mouse/headset battery indicators in Settings → Menu bar, off by default.
  Enabling them detects connected accessories automatically; missing battery data
  displays a dash. Disabling them stops the isolated reader and all accessory
  queries. The landing's Settings demo includes the switch and inline indicators.
  Hardware compatibility still needs validation.
- The menu bar now follows Windows' Battery percentage setting, showing the
  charge next to the battery icon when enabled. Preference changes apply
  without restarting Muelle, through registry notifications rather than polling.
- Fixed icons remaining over the desktop after dragging an app out of a dock
  group. Drag tracking now recovers missed pointer messages without taking
  focus, stops on release or Escape, and rejects late clicks after cancellation.
- Grouping now requires dropping over the target icon vertically as well as
  horizontally. Recovered or collapsed groups no longer duplicate pinned apps
  in recent apps.
- Fixed (issue #15): the Recycle Bin could keep its full icon after being
  emptied, even after restarting Muelle. Right after a change, Explorer may
  still draw the previous state, and that icon was cached for the day under
  the other state. The empty and full icons now come from where Windows names
  them (`DefaultIcon`, including an icon you chose in Desktop icon settings),
  and wrongly cached ones are replaced on their own.

### Español

- Adaptadores USB iniciales para receptores Logitech Unifying/Bolt y variantes
  SteelSeries Nova 7 identificadas. Consultas en procesos desechables, pausadas
  con la barra oculta, con intervalo de 60 segundos y espera tras errores.
  Los niveles aproximados siguen sin porcentaje. Falta validación física; los
  cambios del enlace radio pueden tardar hasta la consulta siguiente.
- Indicadores opcionales de batería del ratón y auriculares en Ajustes → Barra
  superior, desactivados por defecto. Al activarlos se detectan los accesorios
  conectados automáticamente; un guion indica batería no disponible. Desactivarlos
  termina el lector aislado y todas las consultas de accesorios. La demo de Ajustes
  de la landing incluye el interruptor y los indicadores en la barra.
  La compatibilidad con hardware real queda pendiente de validar.
- La barra superior respeta la opción Porcentaje de batería de Windows y
  muestra la carga junto al icono cuando está activada. Los cambios se aplican
  sin reiniciar Muelle, mediante notificaciones del registro, sin sondeo.
- Corregido el icono que quedaba flotando sobre el escritorio al sacar una app
  de un grupo del dock. El arrastre recupera mensajes del puntero perdidos sin
  tomar el foco, termina al soltar o pulsar Escape y descarta clics tardíos tras
  una cancelación.
- Agrupar requiere soltar sobre el icono también en vertical. Recuperar o
  reducir un grupo ya no duplica apps fijadas en las apps recientes.
- Corregido (issue #15): la Papelera podía seguir con el icono de llena tras
  vaciarla, incluso al reiniciar Muelle. Justo después de un cambio, Explorer
  puede dibujar aún el estado anterior, y ese icono se guardaba en caché todo
  el día bajo el otro estado. Los iconos de vacía y llena se leen ahora de
  donde Windows los define (`DefaultIcon`, incluido un icono elegido en la
  configuración de iconos del escritorio), y los guardados mal se reemplazan solos.

## [0.1.15]

### English

- Dragging apps out of the dock or a group now completes or cancels reliably
  even when a non-activating window misses mouse input outside its bounds.
  Press Escape to cancel a drag.
- Dock groups: drag a pinned app onto the middle of another one (a blue frame
  shows it) to put both in a group that takes a single place in the dock;
  drop more apps on the group to add them. A group shows its first apps'
  icons on a framed tile, and lists their names on hover. Clicking it fans
  its apps out above it (a grid when there are many); drag one out of the
  fan to take it out of the group, into the dock where it is dropped.
  Right-click a group to ungroup it.
- Shortcuts folder: choose a folder in Settings → Dock and it shows at the
  right of the dock. Clicking it opens its contents as a menu, as a Windows 10
  taskbar toolbar did: folders first, each one a submenu, then files, with
  Explorer's names and icons; choosing an item opens it. Right-click it to
  open the folder, change it or remove it.
- French: the dock, Settings and the installer are now also in French. Choose
  it in Settings → System, or leave Automatic to follow Windows. A language
  Muelle does not know in the settings file now reads as Automatic instead of
  making the file invalid.
- Mascot showcase: the website now previews the crab's reactions to AI limits,
  charging and energy saver, and the hero crab moves between the Notes window
  and dock. Choose a moment to play its scene or let the preview run; reduced
  motion is respected.
- Apps that showed a blank page instead of their icon now show it, as on the
  taskbar: LibreOffice (its windows belong to `soffice.bin`; the dock now
  shows LibreOffice Calc, Writer… with their names and icons), and programs
  whose icon the Windows shell cannot draw although it is valid (one made
  only of PNG images, as some Electron tools write). Icons are read again
  once after updating.

### Español

- Arrastrar apps fuera del dock o de un grupo ahora termina o se cancela de
  forma fiable aunque una ventana que no toma el foco pierda los eventos del
  mouse fuera de sus límites. Pulsa Esc para cancelar el arrastre.
- Grupos en el dock: arrastra una app fijada al centro de otra (un marco azul
  lo indica) para reunirlas en un grupo que ocupa un solo lugar en el dock;
  suelta más apps sobre el grupo para añadirlas. Un grupo muestra los iconos
  de sus primeras apps en un mosaico enmarcado, y sus nombres al pasar el
  puntero. Al hacer clic se despliegan en abanico encima (en cuadrícula si son
  muchas); arrastra una fuera del abanico para sacarla del grupo, al lugar del
  dock donde la sueltes. Clic derecho en un grupo para desagruparlo.
- Carpeta de accesos directos: elige una carpeta en Ajustes → Dock y aparece
  a la derecha del dock. Al hacer clic, su contenido se abre como un menú,
  como una barra de herramientas de la barra de tareas de Windows 10: primero
  las carpetas, cada una un submenú, luego los archivos, con los nombres e
  iconos del Explorador; al elegir un elemento se abre. Clic derecho para
  abrir la carpeta, cambiarla o quitarla.
- Francés: el dock, Ajustes y el instalador también están en francés.
  Elígelo en Ajustes → Sistema, o deja Automático para seguir a Windows. Un
  idioma que Muelle no conoce en el archivo de ajustes ahora se lee como
  Automático en lugar de invalidar el archivo.
- Vista previa de la mascota: la web muestra las reacciones del cangrejo ante
  los límites de IA, la carga y el ahorro de energía; además, en la portada se
  mueve entre la ventana de Notas y el dock. Elige un momento para ver su escena
  o deja que la vista previa avance; se respeta la opción de reducir movimiento.
- Las apps que mostraban una página en blanco en lugar de su icono ahora lo
  muestran, como en la barra de tareas: LibreOffice (sus ventanas pertenecen
  a `soffice.bin`; el dock ahora muestra LibreOffice Calc, Writer… con sus
  nombres e iconos) y los programas cuyo icono el shell de Windows no sabe
  dibujar aunque sea válido (uno hecho solo de imágenes PNG, como escriben
  algunas herramientas de Electron). Los iconos se vuelven a leer una vez
  tras actualizar.

## [0.1.14]

### English

- Clicking an app whose minimized window is still reported as foreground now
  restores it on the first click, including with one app open and minimized
  previews disabled (#12). File drag-hover uses the same native-state check.

- Automatic startup retries up to three times after a failed launch. Existing
  enabled tasks receive the recovery policy without enabling disabled tasks or
  changing the installation they launch. Settings checks the real registration.
- If Explorer has not created its taskbar yet, Muelle retries taking it over
  without blocking the interface or saving a false original taskbar setting.
- Startup failures before graphics initialization are now recorded in the log.
- Dock window previews keep their positions when switching or minimizing
  windows. Newly discovered windows append; closed windows are removed. Normal
  icon activation continues to use the most recently used window.

### Español

- Pulsar el icono de una app minimizada que Windows aún indica como activa
  ahora la restaura al primer clic, incluso con una sola app abierta y las
  miniaturas de minimizadas desactivadas (#12). El arrastre sobre un icono
  utiliza la misma comprobación del estado real de la ventana.

- El inicio automático reintenta hasta tres veces si falla el arranque. Las
  tareas activadas reciben esta recuperación sin reactivar tareas deshabilitadas
  ni cambiar la instalación que ejecutan. Ajustes consulta el registro real.
- Si Explorer todavía no ha creado su barra, Muelle vuelve a intentar ocultarla
  sin bloquear la interfaz ni guardar un estado original incorrecto.
- El registro también recoge fallos anteriores a la inicialización gráfica.

- Las previsualizaciones del dock conservan su posición al cambiar de ventana
  o minimizarla. Las ventanas nuevas se añaden al final y las cerradas se retiran.
  El clic normal del icono sigue recuperando la ventana usada más recientemente.

## [0.1.13]

### English

- The menu bar's keyboard indicator opens a themed layout menu with the
  current keyboard marked and a link to Windows language settings. Changes
  are requested for the original typing window instead of simulating Win+Space.
- Changing Muelle's language refreshes menu bar text even when its data has
  not changed. Menu bar controls retain the standard arrow cursor.
- Menus return focus after a selection without overriding another app the
  user switched to while dismissing them. Auto-hide keeps the menu bar open
  while its keyboard selector is in use.

- With auto-hide enabled, rest the pointer at a display's bottom edge to
  bring the Dock to that display (#9, #11).
- Menu bar redraws no longer cancel a pending top-edge reveal (#10).
- Screen-edge detection reads the cursor on a dedicated timer thread while
  auto-hide is active, including when the cursor is already at the edge.
- Auto-hidden menu bars verify the cursor position before closing on
  mouse-leave notifications, with a pointer check only while the bar is revealed.
- The menu bar reveals after a shorter edge dwell and slides/fades in and
  out using the Windows compositor. Returning during its exit reverses the
  transition; Windows' disabled animation setting is respected.
- Show/hide transitions wait for compositor completion and keep hidden content
  transparent, preventing cached-frame flashes. Auto-hide also applies before
  the menu bar's first frame at startup.
- Closing a dock context menu resets hover and resumes auto-hide when the
  pointer is outside the dock. File drags keep the dock open.
- Dock and Settings writes are serialized. Changing a preference preserves
  recent pin changes and the dock's latest auto-hide setting. Closing
  Settings waits for pending edits to be saved.
- Taking over the Windows taskbar after resume or an Explorer restart runs
  in the background, as it does at startup.

### Español

- El indicador de teclado de la barra abre un menú con el estilo de Muelle,
  marca el teclado activo y permite abrir los ajustes de idioma de Windows.
  El cambio se solicita a la ventana donde se escribía, sin simular Win+Espacio.
- Cambiar el idioma de Muelle actualiza los textos de la barra aunque no
  cambien sus datos. Los controles de la barra mantienen el cursor de flecha.
- Los menús devuelven el foco tras una selección sin imponerse a otra app
  elegida al cerrarlos. La barra permanece visible mientras se usa su
  selector de teclado, incluso con ocultación automática.

- Con la ocultación automática activada, deja el puntero en el borde inferior
  de una pantalla para mostrar el Dock allí (#9, #11).
- Actualizar la barra superior ya no cancela su aparición al dejar el puntero
  en el borde (#10).
- La detección del borde consulta el puntero desde un hilo con temporizador
  mientras está activa la ocultación automática, incluso si ya está en el borde.
- Los avisos de salida de la barra superior verifican la posición real del
  puntero antes de cerrarla, con una comprobación solo mientras está desplegada.
- La barra aparece con una espera menor en el borde y transiciones de
  deslizamiento y opacidad mediante el compositor de Windows. Volver durante
  la salida la invierte; respeta la desactivación de animaciones de Windows.
- La salida espera a que termine realmente la animación del compositor y el
  contenido permanece transparente al ocultarse, evitando destellos de fotogramas
  anteriores. La auto-ocultación se aplica desde el primer fotograma al iniciar.
- Al cerrar un menú contextual fuera del dock se restablece el puntero y
  vuelve a funcionar la ocultación automática. Al arrastrar archivos, el
  dock permanece abierto.
- El dock y Ajustes guardan por turnos. Cambiar una preferencia conserva
  los pines y el estado de ocultación más recientes. Ajustes guarda los
  cambios pendientes antes de cerrarse.
- Recuperar la barra de tareas tras reanudar Windows o reiniciar el
  Explorador se ejecuta en segundo plano, igual que en el inicio.

## [0.1.12]

### English

- Unplugging the charger now plays a short mascot reaction above the dock,
  including when unplugging also enables energy saver. It replaces an ongoing
  charging reaction and temporarily reveals an auto-hidden dock.
- The first interactive installation opens Muelle Settings when it completes.

### Español

- Al desconectar el cargador, la mascota también aparece sobre el dock,
  incluso si se activa el ahorro de energía a la vez. Sustituye la reacción
  de carga si todavía está en curso y muestra temporalmente el dock oculto.
- La primera instalación interactiva abre Ajustes de Muelle al terminar.

## [0.1.11]

### English

- Removed the manual option to import apps pinned to the Windows taskbar from
  the dock's context menu. Changes to Windows taskbar pins still sync
  automatically.
- Clicking an app while a shell flyout has focus accepts native pointer
  activation before opening it. Minimized windows are restored even if the
  subsequent foreground request is refused. Hidden stale windows are skipped.
- Dock button clicks resolve the current pointer position instead of relying
  on a potentially coalesced mouse-move event.
- Minimization flights keep their capture when Windows focuses the next app.
  Deferred restores still cancel when the user switches to a different app.
- The mascot respects the Windows animation preference and stops when
  disabled, entering full screen or changing display geometry. Initial
  power notifications establish a baseline. Power acts start after the
  dock refresh, and hover reactions preserve the act's exit animation.
  Failed creation no longer reveals an auto-hidden dock or consumes the
  charging cooldown. AI quota cues ignore failed or older measurements
  and comparisons between different quota windows.
- Energy-saver notifications now update the dock and menu bar directly from
  the state sent by Windows. Standard and high-savings modes both disable
  glass, and turning the saver off restores the transparency preference.
- Muelle uses an interactive logon task to avoid Windows' discretionary delay
  of `Run` entries. Windows still controls when the process can start.
  Registration uses the Task Scheduler API with normal priority, battery
  support and a `Run` fallback. Enabled registrations migrate only for the
  matching installation. Migration and Settings changes are serialized,
  and task-removal failures are reported.
- Logon tasks use the account's SID, which survives account renames.
  Changes to autostart run off the Settings UI thread.
- Fixed: after turning the PC on, the dock and the menu bar could stay hidden
  and the Windows taskbar showed instead. Right after signing in, the window
  in front is often a hidden shell window the size of the screen (Start,
  Search or the text input host), and the dock took it for a full-screen app.
  Hidden and cloaked windows no longer count as full screen.
- Diagnostics are written off the UI thread through a bounded queue.
  `%LOCALAPPDATA%\Muelle\muelle.log` and its rotated backup each hold up to
  256 KB. The log records errors, full-screen transitions and initialization
  time measured from entering the dock initialization routine.

### Español

- Se eliminó del menú contextual del dock la opción manual para añadir las
  apps fijadas en la barra de tareas de Windows. Los cambios en esos pines
  siguen sincronizándose automáticamente.
- Al pulsar una app mientras un panel del sistema conserva el foco, el dock
  acepta la activación nativa del clic antes de abrirla. Las ventanas
  minimizadas se restauran aunque se rechace la petición posterior de foco.
  Las ventanas ocultas que hayan quedado en la lista se descartan.
- Los clics del dock consultan la posición actual del puntero, evitando usar
  el icono de un movimiento anterior cuando Windows agrupa eventos del ratón.
- El efecto de minimización conserva la captura cuando Windows enfoca la
  siguiente app. Las restauraciones aplazadas se cancelan si el usuario
  cambia a otra aplicación.
- La mascota respeta la opción de animaciones de Windows y se cancela al
  desactivarla, entrar en pantalla completa o cambiar la geometría de pantalla.
  Los avisos iniciales de energía establecen el estado de referencia.
  Sus escenas comienzan después de actualizar el dock y las reacciones al
  puntero conservan la animación de salida. Un fallo al crearla ya no revela
  el dock oculto ni consume la espera entre avisos del cargador. Los avisos
  de IA descartan lecturas fallidas, antiguas o de ventanas de cuota distintas.
- Las notificaciones de ahorro de energía actualizan el dock y la barra
  superior con el estado enviado por Windows. Tanto el modo estándar como
  el ahorro máximo desactivan el cristal. Al desactivarlos, se recupera la
  preferencia de transparencia del sistema.
- Muelle usa una tarea de inicio de sesión interactiva para evitar el retraso
  que Windows puede aplicar a las entradas `Run`. Windows sigue decidiendo
  cuándo puede arrancar el proceso. La tarea se registra con la API del
  Programador, prioridad normal, soporte para batería y respaldo mediante
  `Run`. Se migran los registros activos de la misma instalación. Los cambios
  desde Ajustes y la migración se ejecutan de forma serializada y los errores
  al eliminar tareas se comunican.
- Las tareas usan el SID de la cuenta, que se conserva al cambiar su nombre.
  Cambiar el inicio automático se ejecuta fuera del hilo de Ajustes.
- Corregido: al encender el PC, el dock y la barra superior podían quedarse
  ocultos y se veía la barra de tareas de Windows. Justo al iniciar sesión, la
  ventana al frente suele ser una ventana oculta del shell del tamaño de la
  pantalla (Inicio, Búsqueda o el host de entrada de texto), y el dock la
  tomaba por una app a pantalla completa. Las ventanas ocultas ya no cuentan.
- El registro de diagnóstico se escribe fuera del hilo de la interfaz mediante
  una cola acotada. `%LOCALAPPDATA%\Muelle\muelle.log` y su copia anterior
  ocupan hasta 256 KB cada uno. Guarda errores, cambios de pantalla completa
  y el tiempo medido desde que comienza la inicialización del dock.

## [0.1.10]

### English

- Right-clicking an empty area of the dock now opens Task Manager or Taskbar
  settings.
- Apps with multiple windows have a dedicated preview picker with more
  reliable hover and foreground handling.
- The Genie effect follows Windows' "Animation effects" setting, which
  "Adjust for best performance" also turns off, and did so silently: the
  switch in Settings stayed on and nothing happened. Settings now says when
  Windows animations are off and opens the right Windows page to turn them on.
- A pixel crab comes out for moments worth it, and only then: it hops with a
  heart under an AI provider whose limit just reset, trembles with a drop
  past 90 %, falls asleep at 100 %, scurries along the dock when the charger
  is plugged in, and dozes off on it when energy saver starts. Hover over it
  and it reacts. An auto-hidden dock comes up for the moment, as the macOS
  Dock does. While no crab is out it costs nothing; its acts run in the system
  compositor. It can be turned off in Settings → Dock.
- Fixed (issue #8): apps pinned from a Windows shortcut with arguments, such
  as `wscript.exe "script.vbs"`, a browser profile or a web app, opened the
  program alone. They now open through their shortcut, exactly as from
  Explorer or the taskbar, and show the shortcut's name and icon. A shortcut
  dropped on the dock is kept in `%APPDATA%\Muelle\Pins`, so the pin survives
  deleting the original; shortcuts to scripts or documents can be pinned too.
- Dragging a file over the dock reads a dropped shortcut once, not on every
  mouse move.

### Español

- Al hacer clic derecho en un espacio vacío del dock, ahora puedes abrir el
  Administrador de tareas o la configuración de la barra de tareas.
- Las apps con varias ventanas tienen un selector de vistas previas propio,
  con un manejo más fiable del puntero y de la ventana en primer plano.
- El efecto genio sigue el ajuste «Efectos de animación» de Windows, que también
  apaga «Ajustar para obtener el mejor rendimiento», y lo hacía en silencio: el
  interruptor de Ajustes seguía activo y no pasaba nada. Ahora Ajustes avisa
  cuando las animaciones de Windows están desactivadas y abre la página de
  Windows correcta para activarlas.
- Un cangrejo pixelado aparece en los momentos que lo merecen, y solo
  entonces: salta con un corazón bajo el proveedor de IA cuyo límite se acaba
  de reiniciar, tiembla con una gota al pasar del 90 %, se duerme al llegar al
  100 %, cruza el dock corriendo al enchufar el cargador y se echa una siesta
  encima al activarse el ahorro de energía. Si pasas el puntero por encima,
  reacciona. El dock oculto sube durante ese momento, como hace el Dock de
  macOS. Mientras no hay cangrejo no consume nada; sus escenas las ejecuta el
  compositor del sistema. Se desactiva en Ajustes → Dock.
- Corregido (issue #8): las apps fijadas desde un acceso directo de Windows con
  argumentos, como `wscript.exe "script.vbs"`, un perfil de navegador o una
  app web, abrían solo el programa. Ahora se abren a través de su acceso
  directo, igual que desde el Explorador o la barra de tareas, y muestran su
  nombre e icono. Un acceso directo soltado en el dock se guarda en
  `%APPDATA%\Muelle\Pins`, así que la app fijada sigue funcionando aunque se
  borre el original; también se pueden fijar accesos a scripts o documentos.
- Al arrastrar un archivo sobre el dock, un acceso directo se lee una sola vez,
  no en cada movimiento del ratón.

## [0.1.9]

### English

- Fixed: bringing back a minimized app from the dock could leave its window
  behind the one in front, as if it had closed again (Warp, for one). Since
  0.1.8 the dock restored the window without taking the focus first, and
  Windows then places it behind the active window.
- GitHub Copilot usage was removed from the menu bar island. Its sign-in is
  only stored encrypted, and decrypting another app's secrets is what made
  antivirus programs flag Muelle; Muelle no longer reads the Windows
  Credential Manager either.
- Fixed: some antivirus programs flagged Muelle 0.1.6–0.1.8 as a trojan and
  deleted it, often right after updating. Two things looked like malware: to
  show GitHub Copilot and Cursor usage, Muelle decrypted VS Code's stored
  secrets and read Cursor's database (what password stealers do), and since
  0.1.7 it set itself up to start with Windows by running `schtasks.exe`
  hidden. Muelle no longer does either: it starts from the usual `Run`
  registry entry (an existing startup task is moved there once), Copilot
  usage comes only from the Copilot CLI sign-in or `GH_TOKEN`/`GITHUB_TOKEN`,
  and Cursor usage was removed. The dock is also 28% smaller.
- Fixed: after choosing an item in the dock's menu (Muelle Settings, for
  one), the dock could stay magnified and stop reacting to the pointer.
- Alt+Tab: the hidden dock shows only for Alt+Tab, no longer for Task View,
  whose virtual desktops it covered at the bottom of the screen; and it no
  longer flashes for a moment after cancelling Alt+Tab with Esc.
- Alt+Tab: moving the mouse or clicking an empty spot no longer stops the
  dock's magnifier from following the selection, and closing a window from
  the switcher keeps it on the right app. Only the arrow keys, which move the
  selection in ways Muelle cannot see, make it step aside.
- Automatic updates: once a day, a couple of minutes after you turn on the
  computer (or every 24 hours if it stays on), Muelle installs new versions by
  itself; the dock restarts in a moment. Never while an app is full screen. It can be turned off in Settings → System → "Install
  updates automatically". The dock itself still never uses the network.
- Windows 10: the menu bar no longer shows a Widgets button, which Windows
  10 does not have; it opened Windows Ink instead.
- Glass follows energy saver: turning it on or off (by hand or at low battery)
  switches the dock, the menu bar and the dock menu between glass and solid
  at once.

### Español

- Corregido: al recuperar una app minimizada desde el dock, su ventana podía
  quedar detrás de la que estaba delante, como si se hubiera cerrado otra vez
  (Warp, por ejemplo). Desde la 0.1.8 el dock restauraba la ventana sin tomar
  antes el foco, y Windows la coloca entonces detrás de la ventana activa.
- Se quitó el uso de GitHub Copilot de la isla de la barra superior. Su sesión
  solo se guarda cifrada, y descifrar los secretos de otra app es lo que hacía
  que los antivirus marcaran Muelle; Muelle tampoco lee ya el Administrador de
  credenciales de Windows.
- Corregido: algunos antivirus marcaban Muelle 0.1.6–0.1.8 como troyano y lo
  borraban, a menudo justo al actualizar. Dos cosas parecían malware: para
  mostrar el uso de GitHub Copilot y Cursor, Muelle descifraba los secretos
  guardados de VS Code y leía la base de datos de Cursor (lo que hacen los
  ladrones de contraseñas), y desde la 0.1.7 se configuraba para iniciar con
  Windows ejecutando `schtasks.exe` oculto. Muelle ya no hace ninguna de las
  dos: arranca desde la entrada habitual `Run` del registro (una tarea de
  inicio existente se traslada ahí una vez), el uso de Copilot sale solo de la
  sesión de Copilot CLI o de `GH_TOKEN`/`GITHUB_TOKEN`, y se quitó el uso de
  Cursor. Además, el dock ocupa un 28 % menos.
- Corregido: tras elegir una opción del menú del dock (por ejemplo, Ajustes
  de Muelle), el dock podía quedarse ampliado y dejar de responder al puntero.
- Alt+Tab: el dock oculto aparece solo con Alt+Tab, ya no con la Vista de
  tareas, cuyos escritorios virtuales tapaba abajo; y ya no parpadea un
  momento después de cancelar Alt+Tab con Esc.
- Alt+Tab: mover el ratón o hacer clic en un hueco ya no impide que la lupa
  del dock siga la selección, y cerrar una ventana desde el selector la
  mantiene en la app correcta. Solo las flechas, que mueven la selección de
  formas que Muelle no ve, hacen que se aparte.
- Actualizaciones automáticas: una vez al día, un par de minutos después de
  encender el ordenador (o cada 24 horas si sigue encendido), Muelle instala
  las versiones nuevas por sí solo; el dock se reinicia en un momento. Nunca
  con una app a pantalla completa. Se puede desactivar en Ajustes →
  Sistema → «Instalar actualizaciones automáticamente». El dock en sí sigue
  sin usar la red.
- Windows 10: la barra superior ya no muestra un botón de Widgets, que Windows
  10 no tiene; abría Windows Ink en su lugar.
- El cristal sigue al ahorro de energía: al activarlo o desactivarlo (a mano o
  con batería baja), el dock, la barra superior y el menú del dock cambian
  entre cristal y sólido al momento.

## [0.1.8]

### English

- Opening apps and shell file operations run in temporary helper processes so
  slow shell handlers and native progress dialogs keep the dock responsive.
  Transfers run in order; ordinary launch helpers time out after 30 seconds,
  while file operations and administrator launches keep their native user controls.
- Recycle Bin notifications share one drive scan at a time, preserving changes
  that arrive during a scan without starting overlapping disk queries.
- Shell icons and app names load in the background through a bounded queue,
  keeping the dock responsive while the shell is slow. Repeated requests share
  the same load, and unused pixel buffers are released.
- Minimize animations prepare restore images off the UI thread and reuse
  thumbnail dimensions between window events. Batch minimization starts one
  capture; a stalled capture no longer keeps an overlay and frame loop alive.
- Drag and drop into apps: hold a file (or text, or an image) over an open
  app in the dock and its window comes to the front, so you can drop it there,
  as with the Windows taskbar. Passing over icons does not switch windows.
- Fixed: with "Automatically hide and show the menu bar" on, the bar covered the
  minimize, maximize and close buttons of maximized windows and took clicks
  meant for browser tabs at the top edge. The hidden bar no longer keeps any
  window on screen; it appears after the pointer rests briefly on the top
  edge, and never over a window's title-bar buttons.
- The top-right corner of the menu bar no longer shows the desktop: throwing
  the pointer there to close a window minimized everything. "Show Desktop"
  remains in the dock menu.
- Fixed: clicking the menu bar's "^" again left the hidden-icons popup open
  and moved the pointer to its first icon. The second click now closes it.
- The dock's menu looks like a macOS menu: frosted glass in the light or dark
  theme, compact rows and the selection in your Windows accent color. It
  works fully from the keyboard; with a screen reader or high contrast on,
  the native Windows menu is used instead.
- An auto-hidden dock shows while Alt+Tab (or Task View) is open, so the
  open apps are in view, and hides again when you let go. As you press Tab,
  the dock magnifies the app of the selected window. Windows does not report
  the selection, so it follows Tab and Shift+Tab; after arrow keys or the
  mouse in the switcher, the magnifier steps aside rather than guess.
- The menu bar no longer shows a location icon. Widgets keeps location in use
  for the weather all the time, and Windows shows no icon for that either.
- Windows 10: apps pinned to the taskbar from the Microsoft Store (Store, Mail
  and others) now come to the dock. Versions before 0.1.7 could not import
  them, and updating did not add them afterwards.

### Español

- Abrir apps y ejecutar operaciones de archivos del shell se hace en procesos
  auxiliares temporales para mantener la respuesta del dock mientras el shell
  tarda o muestra el progreso. Las transferencias se ejecutan en orden; las
  aperturas normales tienen un límite de 30 segundos, mientras las operaciones
  de archivos y las aperturas como administrador conservan sus controles nativos.
- Los avisos de Papelera comparten una sola lectura de discos a la vez y
  conservan los cambios que lleguen durante esa lectura, sin consultas solapadas.
- Los iconos y nombres del shell se cargan en segundo plano con una cola
  limitada para que el dock responda mientras el shell tarda. Las solicitudes
  repetidas comparten la misma carga y se liberan los píxeles que ya no se usan.
- Las animaciones de minimizar preparan la imagen de restauración fuera del
  hilo de interfaz y reutilizan las dimensiones de las miniaturas entre avisos
  de ventanas. Al minimizar en lote se inicia una captura; una captura atascada
  ya no mantiene una vista previa y un ciclo de fotogramas indefinidos.
- Arrastrar y soltar en las apps: mantén un archivo (o texto, o una imagen)
  sobre una app abierta del dock y su ventana pasa al frente para que lo
  sueltes ahí, como con la barra de tareas de Windows. Pasar por encima de los
  iconos no cambia de ventana.
- Corregido: con «Ocultar y mostrar la barra superior automáticamente», la barra tapaba
  los botones de minimizar, maximizar y cerrar de las ventanas maximizadas y
  se quedaba con los clics en las pestañas del navegador del borde superior.
  La barra oculta ya no deja ninguna ventana en pantalla: aparece cuando el
  puntero se detiene un momento en el borde superior, y nunca encima de los
  botones de una ventana.
- La esquina superior derecha de la barra ya no muestra el escritorio: lanzar
  el puntero ahí para cerrar una ventana minimizaba todo. «Mostrar escritorio»
  sigue en el menú del dock.
- Corregido: volver a pulsar «^» en la barra superior dejaba abierto el
  panel de iconos ocultos y movía el puntero a su primer icono. Ahora el
  segundo clic lo cierra.
- El menú del dock se ve como un menú de macOS: cristal esmerilado en tema
  claro u oscuro, filas compactas y la selección en el color de énfasis de
  Windows. Funciona por completo con el teclado; con un lector de pantalla o
  alto contraste activos se usa el menú nativo de Windows.
- El dock oculto aparece mientras Alt+Tab (o la Vista de tareas) está abierto,
  para ver las apps abiertas, y se vuelve a ocultar al soltar. Al pulsar Tab,
  el dock amplía la app de la ventana seleccionada. Windows no informa de la
  selección, así que sigue Tab y Mayús+Tab; si se usan las flechas o el ratón
  en el selector, la lupa se aparta en vez de adivinar.
- La barra superior ya no muestra el icono de ubicación. Widgets mantiene la
  ubicación en uso todo el tiempo para el clima, y Windows tampoco muestra un
  icono por eso.
- Windows 10: las apps de Microsoft Store fijadas en la barra de tareas
  (Store, Correo y otras) ahora llegan al dock. Las versiones anteriores a la
  0.1.7 no podían importarlas, y al actualizar no se añadían después.

## [0.1.7]

### English

- Faster startup: Muelle now uses a Windows sign-in task instead of the Run
  key, and keeps the taskbar hidden when Windows shuts down or restarts.
- Taskbar pins stay in sync with the dock, including changes made while Muelle
  is closed. Imported pins keep their order and include Store apps; removed
  apps no longer remain as broken dock icons.
- The dock menu can import apps pinned to the Windows taskbar. Unchecking
  "Keep in Dock" also unpins the app from the taskbar.
- Middle-click or Shift-click opens another app window; Ctrl+Shift-click or
  the dock menu opens an app as administrator. The menu bar's right edge and
  the dock menu can show the desktop.
- The menu bar shows the active input language and privacy indicators for
  microphone, camera, and location use. Its clock follows time and time-zone
  changes, and switching virtual desktops updates the dock immediately.
- Genie: maximized and snapped windows now fly using the area they occupy on
  screen, without jumping to a different size.

### Español

- Inicio más rápido: Muelle ahora usa una tarea de Windows al iniciar sesión
  en lugar de la clave Run y mantiene oculta la barra de tareas al apagar o
  reiniciar Windows.
- Las apps fijadas en la barra de tareas se sincronizan con el dock, incluso
  si el cambio ocurre mientras Muelle está cerrado. Las apps importadas
  conservan su orden e incluyen las de la Store; las desinstaladas ya no quedan
  como iconos rotos.
- El menú del dock permite importar las apps fijadas en la barra de tareas.
  Desmarcar «Mantener en el Dock» también desancla la app de la barra.
- El clic central o Mayús+clic abre otra ventana; Ctrl+Mayús+clic o el menú del
  dock abre una app como administrador. El borde derecho de la barra superior
  y el menú del dock permiten mostrar el escritorio.
- La barra superior muestra el idioma de entrada activo e indicadores cuando
  se usan el micrófono, la cámara o la ubicación. El reloj sigue los cambios
  de hora y zona horaria, y al cambiar de escritorio virtual el dock se
  actualiza de inmediato.
- Genio: las ventanas maximizadas o acopladas vuelan usando el área que ocupan
  en pantalla, sin saltar a otro tamaño.

## [0.1.6]

### English

- Windows 10 support (version 1903 or later). The dock used to close right
  after starting there. On Windows 10 the dock and menu bar are solid instead
  of glass, since it has no backdrop blur for desktop apps.
- The menu bar's "^" button opens Windows' hidden notification-area icons.
- A curved menu bar island shows usage for Claude, Codex, GitHub Copilot,
  Cursor, OpenCode Go, and Gemini or Antigravity while they are running. It can
  be turned off in Settings → Menu bar.
- Settings has a clearer layout with separate Dock, Windows, Menu bar and
  System sections, and its window can be resized or maximized.
- Minimized windows now fly into their app's dock icon by default. Live
  thumbnails on the right side of the dock remain available as an option.
- The active-app click option was removed: clicking the app in front minimizes
  it, as on the Windows taskbar.
- The dock's Muelle icon is sharper, and the Start button uses the Windows
  logo.
- Settings can turn off automatic update checks. The website adds privacy and
  code-signing policy pages.

### Español

- Soporte para Windows 10 (versión 1903 o posterior). Antes el dock se cerraba
  nada más arrancar ahí. En Windows 10 el dock y la barra superior son sólidos
  en lugar de cristal, porque no ofrece desenfoque de fondo a las apps de
  escritorio.
- El botón «^» de la barra superior abre los iconos ocultos del área de
  notificación de Windows.
- Una isla curva en la barra superior muestra el uso de Claude, Codex, GitHub
  Copilot, Cursor, OpenCode Go y Gemini o Antigravity mientras están abiertos.
  Se puede desactivar en Ajustes → Barra superior.
- Ajustes tiene secciones separadas para Dock, Ventanas, Barra superior y
  Sistema; su ventana se puede redimensionar o maximizar.
- Por defecto, las ventanas minimizadas vuelan hacia el icono de su app en el
  dock. Las miniaturas en vivo a la derecha siguen disponibles como opción.
- Se quitó la opción de clic en la app activa: al hacer clic en la app que está
  delante, se minimiza, como en la barra de Windows.
- El icono de Muelle se ve más nítido en el dock y el botón Inicio usa el logo
  de Windows.
- Ajustes permite desactivar la búsqueda automática de actualizaciones. La web
  incorpora páginas de privacidad y firma de código.

## [0.1.5]

### English

- The dock appears sooner at startup (about 170 ms instead of 260-450 ms).
- Lower idle CPU: windows changing their title (terminals, browsers) no longer
  make the dock re-read every window.
- Genie starts sooner after minimizing: the screen copy no longer waits extra
  frames.
- Fixed: the dock and menu bar could stay black after energy saver; without
  transparency they are now solid gray, as macOS draws them with "Reduce
  transparency".
- Fixed: the menu bar could miss an update and showed no app name until the
  first window switch.
- Menu bar: the battery shows a charging bolt.
- Settings: options that depend on another fold away under it.

### Español

- El dock aparece antes al arrancar (unos 170 ms en lugar de 260-450 ms).
- Menos CPU en reposo: las ventanas que cambian de título (terminales,
  navegadores) ya no hacen que el dock relea todas las ventanas.
- El genio empieza antes al minimizar: la copia de pantalla ya no espera
  fotogramas de más.
- Corregido: el dock y la barra superior podían quedarse negros tras el ahorro
  de energía; sin transparencia ahora son grises sólidos, como macOS con
  «Reducir transparencia».
- Corregido: la barra superior podía perder una actualización y no mostraba el
  nombre de la app hasta el primer cambio de ventana.
- Barra superior: la batería muestra un rayo mientras carga.
- Ajustes: las opciones que dependen de otra se pliegan bajo ella.

## [0.1.4]

### English

- Maintenance release: align Genie default documentation and tests with the
  shipped setting.
- Website: add an Open Graph preview image and document the observed idle
  memory reading.

### Español

- Versión de mantenimiento: alinear la documentación y las pruebas del efecto
  Genio con el ajuste publicado.
- Web: añadir una imagen de vista previa Open Graph y documentar la lectura
  observada de memoria en reposo.

## [0.1.2]

### English

- Updates are announced in Settings: a banner above every section with
  "Update now", "Later" and "What's new", download progress, and a badge on
  System while an update waits.
- Smoother Genie effect: the system compositor now runs the whole animation
  at the display's refresh rate, and it no longer skips its first frames.
- Genie: no more flash when minimizing, and a window restored with Alt+Tab or
  from the app itself animates again the next time it is minimized.
- Fixed: the Dock section of Settings could not scroll to its last options.
- Fixed: opening Settings twice opened two windows; it now brings the open one
  to the front.
- Fixed: with an app running as administrator in front (Task Manager, an admin
  terminal, an installer), the dock's Start button and the menu bar buttons did
  nothing, because Windows drops simulated keys aimed at such apps.

### Español

- Las actualizaciones se anuncian en Ajustes: un aviso encima de cada sección
  con «Actualizar ahora», «Más tarde» y «Novedades», progreso de la descarga y
  una insignia en Sistema mientras haya una pendiente.
- Efecto genio más fluido: el compositor del sistema ejecuta toda la animación
  a la frecuencia del monitor y ya no se salta sus primeros fotogramas.
- Genio: ya no hay destello al minimizar, y una ventana restaurada con Alt+Tab o
  desde la propia app vuelve a animarse la siguiente vez que se minimiza.
- Corregido: la sección Dock de Ajustes no dejaba bajar hasta las últimas opciones.
- Corregido: abrir Ajustes dos veces abría dos ventanas; ahora trae al frente la abierta.
- Corregido: con una app de administrador delante (Administrador de tareas, una
  terminal de administrador, un instalador), el botón Inicio del dock y los botones
  de la barra superior no hacían nada, porque Windows descarta las teclas simuladas
  dirigidas a esas apps.

## [0.1.1]

### English

- Menu bar: network (Wi-Fi strength, Ethernet or offline) and volume, updated
  as they change. Search, Task View and Widgets buttons appear when they are on
  in your Windows taskbar settings.
- Clicking the app that is already in front minimizes it, as the Windows
  taskbar does; click again to bring it back. Settings can switch this to
  "do nothing", as on a Mac.
- Apps opened from the dock come to the front reliably instead of sometimes
  opening behind other windows.
- Minimized apps that run as administrator, such as Task Manager, can be brought
  back from the dock.
- Settings: restore defaults, and options to show open-app indicators, show
  recent apps and animate opening apps.

### Español

- Barra superior: red (intensidad del Wi-Fi, Ethernet o sin conexión) y volumen,
  actualizados al momento. Los botones de búsqueda, Vista de tareas y Widgets
  aparecen si los tienes activados en la barra de tareas de Windows.
- Clic en la app que ya está delante la minimiza, como la barra de Windows; otro
  clic la vuelve a traer. En Ajustes se puede cambiar a «no hacer nada», como en Mac.
- Las apps abiertas desde el dock aparecen delante siempre, en vez de abrirse a
  veces detrás de otras ventanas.
- Las apps minimizadas que se ejecutan como administrador, como el Administrador
  de tareas, se pueden recuperar desde el dock.
- Ajustes: restablecer valores y opciones para mostrar los indicadores de apps
  abiertas, mostrar las apps recientes y animar al abrir apps.

## [0.1.0]

### English

- First release: glass dock with magnification, live minimized-window
  thumbnails, optional Genie effect, menu bar, Settings app in English and
  Spanish, one-step installer and in-app updates.

### Español

- Primera versión: dock de cristal con ampliación, miniaturas en vivo de las
  ventanas minimizadas, efecto genio opcional, barra superior, app de Ajustes en
  español e inglés, instalador de un paso y actualizaciones desde la app.
