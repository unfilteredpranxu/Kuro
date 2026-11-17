mod store;
use dioxus::prelude::*;
use kuro_core::*;
use store::*;
const NAV: [(&str, &str); 12] = [
    ("Home", "01"),
    ("Syllabus", "02"),
    ("Planner", "03"),
    ("Tasks", "04"),
    ("Library", "05"),
    ("Notes", "06"),
    ("Practice", "07"),
    ("Revision", "08"),
    ("Exams", "09"),
    ("Music", "10"),
    ("Analytics", "11"),
    ("Settings", "12"),
];
fn main() {
    dioxus::launch(App);
}
#[component]
fn App() -> Element {
    let state = use_signal(State::default);
    let busy = use_signal(|| false);
    let ready = use_signal(|| false);
    let error = use_signal(String::new);
    let message = use_signal(String::new);
    let mut store = AppStore {
        state,
        busy,
        ready,
        error,
        message,
    };
    let mut page = use_signal(|| "Home".to_string());
    let mut palette = use_signal(|| false);
    let mut query = use_signal(String::new);
    let mut collapsed = use_signal(|| false);
    use_effect(move || dialog_focus(palette()));
    use_effect(move || {
        shortcuts();
        spawn(async move {
            match load().await {
                Ok(raw) => {
                    let text = raw.as_string().unwrap_or_default();
                    let parsed = if text.is_empty() {
                        Ok(State::default())
                    } else {
                        serde_json::from_str::<State>(&text)
                            .map_err(|e| e.to_string())
                            .and_then(|s| {
                                validate(&s)?;
                                Ok(s)
                            })
                    };
                    match parsed {
                        Ok(s) => {
                            store.state.set(s);
                            store.ready.set(true);
                        }
                        Err(e) => {
                            store
                                .error
                                .set(
                                    format!(
                                        "AppStored data needs recovery: {e}. Original data was not overwritten.",
                                    ),
                                )
                        }
                    }
                }
                Err(e) => store.error.set(describe(e)),
            }
        });
    });
    let theme = store.state.read().theme.to_lowercase();
    let disabled = !*ready.read() || *busy.read();
    rsx! {
        div { class: "app theme-{theme}", "data-collapsed": "{collapsed}",
            a { class: "skip", href: "#workspace", "Skip to workspace" }
            aside { class: "sidebar", "aria-label": "Main navigation",
                button { class: "brand", onclick: move | _ |
                            page.set("Home".into()),
                    span { "黒" }
                    div {
                        strong { "KURO" }
                        small { "STUDY OPERATING SYSTEM" }
                    }
                }
                div { class: "sidebar-caption", "WORKSPACE / 2027" }
                nav {
                    for (name, number) in NAV {
                        button {
                            class: if page() == name { "nav-item active" } else { "nav-item" },
                            "aria-label": "{number} {name}",
                            "aria-current": if page() == name { "page" } else { "false" },
                            onclick: move |_| {
                                page.set(name.into());
                            },
                            span { class: "nav-number", "{number}" }
                            span { "{name}" }
                            if page() == name {
                                span { class: "nav-arrow", "↗" }
                            }
                        }
                    }
                }
                div { class: "sidebar-bottom",
                    span { class: "online-dot" }
                    "LOCAL-FIRST / PRIVATE"
                    p { "One day. One concept.\nA little closer." }
                    button {
                        class: "text-button collapse",
                        onclick: move |_| collapsed.toggle(),
                        if collapsed() {
                            "→ Expand"
                        } else {
                            "← Compact"
                        }
                    }
                }
            }
            div { class: "main-shell",
                header { class: "topbar",
                    div { class: "breadcrumb",
                        span { "PERSONAL WORKSPACE" }
                        b { "/" }
                        strong { "{page}" }
                    }
                    div { class: "topbar-actions",
                        span { class: "timezone", "IST · ASIA/KOLKATA" }
                        button {
                            id: "command-toggle",
                            class: "command-button",
                            onclick: move |_| palette.toggle(),
                            "Search"
                            kbd { "⌘ K" }
                        }
                    }
                }
                main { id: "workspace",
                    if !error().is_empty() {
                        div { role: "alert", class: "error-banner",
                            strong { "NOT SAVED / " }
                            "{error}"
                            button { onclick: move |_| store.error.set(String::new()),
                                "Dismiss"
                            }
                        }
                    }
                    div { class: "save-status", role: "status",
                        if busy() {
                            "SAVING…"
                        } else if ready() {
                            "{message}"
                        } else {
                            "OPENING DEVICE STORAGE…"
                        }
                    }
                    match page().as_str() {
                        "Home" => rsx! {
                            Home { store, page }
                        },
                        "Syllabus" => rsx! {
                            Syllabus {}
                        },
                        "Tasks" => rsx! {
                            Tasks { store, disabled }
                        },
                        "Planner" => rsx! {
                            Planner { store, page }
                        },
                        "Notes" => rsx! {
                            Notes { store, disabled }
                        },
                        "Practice" => rsx! {
                            Practice { store, disabled }
                        },
                        "Revision" => rsx! {
                            Revision { store, disabled }
                        },
                        "Analytics" => rsx! {
                            Analytics { store }
                        },
                        "Settings" => rsx! {
                            Settings { store, disabled }
                        },
                        "Music" => rsx! {
                            Music {}
                        },
                        name => rsx! {
                            Unavailable { name: name.to_string() }
                        },
                    }
                    footer {
                        span { "KURO // STUDY OS" }
                        span { "BUILD 0.1 · WEB FOUNDATION" }
                        span { "静かに、着実に。" }
                    }
                }
            }
            if palette() {
                div {
                    class: "modal-backdrop",
                    onclick: move | _ | palette
                            .set(false),
                    section {
                        role: "dialog",
                        "aria-modal": "true",
                        "aria-label": "Command palette",
                        class: "palette",
                        onclick: move |e| e.stop_propagation(),
                        onkeydown: move |e| {
                            if e.key() == Key::Escape {
                                palette.set(false);
                            }
                        },
                        div { class: "panel-heading",
                            h2 { "Go somewhere." }
                            button {
                                "aria-label": "Close search",
                                onclick: move |_| palette.set(false),
                                "×"
                            }
                        }
                        input {
                            autofocus: true,
                            placeholder: "Find a module…",
                            "aria-label": "Search modules",
                            value: "{query}",
                            oninput: move |e| query.set(e.value()),
                        }
                        for (name, number) in NAV.into_iter()
                            .filter(|(name, _)| name.to_lowercase().contains(&query().to_lowercase()))
                        {
                            button {
                                class: "palette-item",
                                onclick: move |_| {
                                    page.set(name.into());
                                    palette.set(false);
                                    query.set(String::new());
                                },
                                span { "{number} / {name}" }
                                span { "↗" }
                            }
                        }
                    }
                }
            }
        }
    }
}
#[component]
fn PageTitle(kicker: String, title: String, subtitle: String) -> Element {
    rsx! {
        div { class: "page-title",
            div { class: "eyebrow", "{kicker}" }
            h1 { "{title}" }
            p { "{subtitle}" }
        }
    }
}
#[component]
fn Home(store: AppStore, mut page: Signal<String>) -> Element {
    let s = store.state.read();
    let open = s.tasks.iter().filter(|t| !t.done).count();
    let due = s.cards.iter().filter(|c| c.due <= today()).count();
    let acc = accuracy(&s.attempts)
        .map(|x| format!("{x:.0}%"))
        .unwrap_or("—".into());
    let day = today();
    rsx! {
        section { class: "home-heading",
            div {
                div { class: "eyebrow", "YOUR QUIET CORNER OF THE INTERNET" }
                h1 {
                    "Make room"
                    br {}
                    em { "for understanding." }
                }
                p { "A deliberate space for a difficult, worthwhile journey." }
            }
            div { class: "date-stamp",
                span { "STUDY LOG / IST" }
                strong { "{day}" }
                span { "JEE ADVANCED · 2027" }
            }
        }
        section { class: "hero-grid",
            div {
                class: "hero-art",
                role: "img",
                "aria-label": "Original pixel illustration of a quiet mountain observatory under a crescent moon",
                div { class: "art-caption",
                    span { "NOCTURNE / 01" }
                    span { "夜の静けさ" }
                }
                div { class: "art-bottom",
                    span { "THE LONG WAY IS THE WAY." }
                    span { "27° / NORTH" }
                }
            }
            div { class: "today-panel",
                div { class: "eyebrow", "01 / THE NEXT SMALL STEP" }
                h2 {
                    if open == 0 {
                        "A clean slate.\nA clear intention."
                    } else {
                        "Return to\nwhat matters."
                    }
                }
                p {
                    if open == 0 {
                        "Choose one concept. Give it your attention. Your preparation begins with an honest first step."
                    } else {
                        "Your study tasks are ready. Begin with one, and record what you actually complete."
                    }
                }
                button {
                    class: "primary",
                    onclick: move |_| page.set("Tasks".into()),
                    if open == 0 {
                        "Plan your first session"
                    } else {
                        "Open your tasks"
                    }
                    span { "↗" }
                }
                div { class: "goal-label", "DESTINATION" }
                div { class: "goal",
                    "IIT Bombay"
                    span { "Computer Science & Engineering" }
                }
            }
        }
        section { class: "metrics", "aria-label": "Real study statistics",
            div {
                span { "SYLLABUS COVERAGE" }
                strong { "—" }
                small { "Official source awaiting access" }
            }
            div {
                span { "INDEPENDENT ACCURACY" }
                strong { "{acc}" }
                small { "From your recorded attempts" }
            }
            div {
                span { "OPEN TASKS" }
                strong { "{open:02}" }
                small { "Completion is never assumed" }
            }
            div {
                span { "REVIEWS DUE" }
                strong { "{due:02}" }
                small { "Recall, then record" }
            }
        }
        section { class: "home-lower",
            div { class: "panel",
                div { class: "panel-heading",
                    h2 { "On your desk" }
                    button {
                        class: "text-button",
                        onclick: move |_| page.set("Tasks".into()),
                        "ALL TASKS ↗"
                    }
                }
                if open == 0 {
                    div { class: "empty-inline",
                        span { class: "pixel-cross", "+" }
                        div {
                            h3 { "Nothing scheduled. Yet." }
                            p { "An empty desk is a good place to begin." }
                        }
                    }
                }
                for t in s.tasks.iter().filter(|t| !t.done).take(3) {
                    div { class: "desk-row",
                        span { "□" }
                        div {
                            strong { "{t.title}" }
                            small { "{t.subject} / {t.date}" }
                        }
                        span { "{t.minutes}m" }
                    }
                }
            }
            div { class: "panel journey",
                div { class: "eyebrow", "THE HORIZON" }
                h2 { "23 / 05 / 2027" }
                p { "Your target date for JEE Advanced. Official 2027 confirmation is still required." }
                div { class: "line-scale" }
                span { "CONSISTENCY OVER INTENSITY" }
            }
        }
    }
}
#[component]
fn Syllabus() -> Element {
    rsx! {
        PageTitle {
            kicker: "02 / THE MAP",
            title: "Know the territory.",
            subtitle: "Official requirements, connected to the way you study.",
        }
        div { class: "source-notice",
            span { class: "eyebrow", "SOURCE VERIFICATION BLOCKED" }
            h2 { "An honest map starts with the source." }
            p {
                "The cloud network currently blocks jeeadv.ac.in. No unofficial topic list has been substituted. The 2026 PDF is the requested baseline; a 2027 syllabus has not been verified."
            }
            a {
                class: "primary",
                href: "https://jeeadv.ac.in/documents/jee-advanced-2026-syllabus.pdf",
                target: "_blank",
                rel: "noopener noreferrer",
                "Open official 2026 syllabus ↗"
            }
        }
        div { class: "subject-grid",
            for (n, name, mark) in [("01", "Physics", "φ"), ("02", "Chemistry", "χ"), ("03", "Mathematics", "∑")] {
                div { class: "subject-panel",
                    span { class: "eyebrow", "{n} / OFFICIAL BASELINE 2026" }
                    strong { "{mark}" }
                    h2 { "{name}" }
                    p { "Awaiting complete PDF ingestion and coverage audit." }
                }
            }
        }
        p { class: "muted",
            "Topic status controls will be enabled after all source pages and sections have been audited. Tasks and notes can use your own chapter labels meanwhile."
        }
    }
}
#[component]
fn Tasks(store: AppStore, disabled: bool) -> Element {
    let mut title = use_signal(String::new);
    let mut subject = use_signal(|| "Physics".to_string());
    let mut date = use_signal(today);
    let mut minutes = use_signal(|| "120".to_string());
    let mut topic = use_signal(String::new);
    let mut filter = use_signal(|| "Open".to_string());
    let tasks = store.state.read().tasks.clone();
    rsx! {
        PageTitle {
            kicker: "04 / INTENTION INTO ACTION",
            title: "One thing at a time.",
            subtitle: "Schedule honestly. Complete deliberately. Carry forward without guilt.",
        }
        div { class: "work-grid",
            section { class: "panel",
                div { class: "panel-heading",
                    h2 { "Your tasks" }
                    span { class: "eyebrow", "{tasks.len()} TOTAL" }
                }
                div { class: "tabs",
                    for name in ["Open", "Today", "Overdue", "Completed", "All"] {
                        button {
                            class: if filter() == name { "selected" } else { "" },
                            onclick: move | _ |
                                    filter.set(name.into()),
                            "{name}"
                        }
                    }
                }
                if tasks.is_empty() {
                    Empty {
                        title: "Space for a first step.",
                        body: "Create a session with a realistic duration and date.",
                    }
                }
                for t in tasks
                    .iter()
                    .filter(|t| match filter().as_str() {
                        "Open" => !t.done,
                        "Today" => t.date == today() && !t.done,
                        "Overdue" => t.date < today() && !t.done,
                        "Completed" => t.done,
                        _ => true,
                    })
                {
                    TaskRow { key: "{t.id}", task: t.clone(), store, disabled }
                }
            }
            section { class: "panel form-panel",
                h2 { "Add a task" }
                form {
                    onsubmit: move |e| {
                        e.prevent_default();
                        let task = Task {
                            id: uid(),
                            title: title().trim().into(),
                            subject: subject(),
                            date: date(),
                            minutes: minutes().parse().unwrap_or(0),
                            done: false,
                            topic: if topic().trim().is_empty() { None } else { Some(topic()) },
                        };
                        commit(store, move |s| s.tasks.push(task));
                    },
                    label {
                        "Task title"
                        input {
                            required: true,
                            maxlength: 500,
                            placeholder: "e.g. Vectors — worked examples",
                            value: "{title}",
                            oninput: move | e | title
                                    .set(e.value()),
                        }
                    }
                    label {
                        "Subject"
                        select {
                            value: "{subject}",
                            onchange: move |e| subject.set(e.value()),
                            for name in ["Physics", "Chemistry", "Mathematics", "English", "Physical Education", "General"] {
                                option { value: "{name}", "{name}" }
                            }
                        }
                    }
                    label {
                        "Chapter / topic label (optional)"
                        input {
                            value: "{topic}",
                            oninput: move |e| topic.set(e.value()),
                            placeholder: "Your own label, until syllabus is audited",
                        }
                    }
                    div { class: "form-pair",
                        label {
                            "Date · IST"
                            input {
                                r#type: "date",
                                required: true,
                                value: "{date}",
                                oninput: move |e| date.set(e.value()),
                            }
                        }
                        label {
                            "Minutes"
                            input {
                                r#type: "number",
                                min: 1,
                                max: 720,
                                required: true,
                                value: "{minutes}",
                                oninput: move |e| minutes.set(e.value()),
                            }
                        }
                    }
                    button { class: "primary", r#type: "submit", disabled,
                        "Create task"
                        span { "+" }
                    }
                }
                p { class: "muted small", "Completing a task does not mark a topic as mastered." }
            }
        }
    }
}
#[component]
fn TaskRow(task: Task, store: AppStore, disabled: bool) -> Element {
    let mut editing = use_signal(|| false);
    let mut date = use_signal(|| task.date.clone());
    let mut title = use_signal(|| task.title.clone());
    let id = task.id.clone();
    let edit_id = id.clone();
    let done = task.done;
    rsx! {
        article { class: if done { "task-row done" } else { "task-row" },
            button {
                class: "check",
                "aria-label": if done { format!("Reopen {}", task.title) } else { format!("Complete {}", task.title) },
                disabled,
                onclick: move |_| {
                    let id = id.clone();
                    commit(
                        store,
                        move |s| {
                            if let Some(t) = s.tasks.iter_mut().find(|t| t.id == id) {
                                t.done = !t.done;
                            }
                        },
                    )
                },
                if done {
                    "✓"
                } else {
                    " "
                }
            }
            div { class: "task-content",
                h3 { "{task.title}" }
                p { "{task.subject} / {task.date} / {task.minutes} MIN" }
                if let Some(topic) = &task.topic {
                    small { "↳ {topic}" }
                }
            }
            button { class: "text-button", onclick: move |_| editing.toggle(), "Edit" }
            if editing() {
                form {
                    class: "inline-edit",
                    onsubmit: move |e| {
                        e.prevent_default();
                        let id = edit_id.clone();
                        let d = date();
                        let name = title();
                        commit(
                            store,
                            move |s| {
                                if let Some(t) = s.tasks.iter_mut().find(|t| t.id == id) {
                                    t.date = d;
                                    t.title = name;
                                }
                            },
                        );
                    },
                    label {
                        "Title"
                        input {
                            required: true,
                            value: "{title}",
                            oninput: move |e| title.set(e.value()),
                        }
                    }
                    label {
                        "Reschedule · IST"
                        input {
                            r#type: "date",
                            required: true,
                            value: "{date}",
                            oninput: move | e | date.set(e
                                    .value()),
                        }
                    }
                    button { class: "secondary", disabled, "Save task" }
                }
            }
        }
    }
}
const ROUTINE: [(&str, &str, &str); 12] = [
    ("08:30", "10:30", "Concepts"),
    ("10:30", "10:45", "Break"),
    ("10:45", "12:45", "Worked examples & concepts"),
    ("12:45", "13:30", "Lunch"),
    ("13:30", "15:00", "Independent question-solving"),
    ("15:00", "15:15", "Break"),
    ("15:15", "17:45", "DPPs & PYQs"),
    ("18:00", "20:00", "Travel, food & personal time"),
    ("20:00", "21:15", "Revision"),
    ("21:15", "21:30", "Break"),
    ("21:30", "22:40", "Timed practice & error review"),
    ("23:00", "07:00", "Sleep"),
];
#[component]
fn Planner(store: AppStore, mut page: Signal<String>) -> Element {
    let mut date = use_signal(today);
    let tasks = store.state.read().tasks.clone();
    rsx! {
        PageTitle {
            kicker: "03 / RHYTHM",
            title: "A day with direction.",
            subtitle: "Asia/Kolkata · Your routine is a guide, never an automatic completion record.",
        }
        div { class: "work-grid",
            section { class: "panel",
                div { class: "panel-heading",
                    h2 { "Daily agenda" }
                    input {
                        r#type: "date",
                        "aria-label": "Agenda date",
                        value: "{date}",
                        oninput: move |e| date.set(e.value()),
                    }
                }
                for t in tasks.iter().filter(|t| t.date == date()) {
                    div { class: "desk-row",
                        span {
                            if t.done {
                                "✓"
                            } else {
                                "□"
                            }
                        }
                        div {
                            strong { "{t.title}" }
                            small { "{t.subject}" }
                        }
                        span { "{t.minutes}m" }
                    }
                }
                if !tasks.iter().any(|t| t.date == date()) {
                    Empty {
                        title: "This day is yours to shape.",
                        body: "Add a dated task. For a two-day chapter, create a separate task for each day; progress stays independent.",
                    }
                }
                button {
                    class: "secondary",
                    onclick: move |_| page.set("Tasks".into()),
                    "Plan a task ↗"
                }
            }
            section { class: "panel",
                h2 { "Default study rhythm" }
                p { class: "muted small",
                    "07:00 wake · 23:00 sleep. Read-only template in this build; dated tasks are editable."
                }
                for (start, end, label) in ROUTINE {
                    div { class: "routine-row",
                        span { "{start}–{end}" }
                        strong { "{label}" }
                    }
                }
            }
        }
    }
}
fn new_note() -> Note {
    Note {
        id: uid(),
        title: String::new(),
        body: String::new(),
        topic: None,
    }
}
fn remember(mut store: AppStore, mut draft: Signal<Note>, change: impl FnOnce(&mut Note)) {
    let Ok(value) = draft.try_read() else {
        return;
    };
    let mut next = value.clone();
    drop(value);
    change(&mut next);
    if let Ok(json) = serde_json::to_string(&next)
        && let Err(e) = save_draft(&json)
    {
        store.error.set(format!(
            "Draft recovery failed: {}. Keep this page open and save your note.",
            describe(e)
        ));
    }
    if let Ok(mut value) = draft.try_write() {
        *value = next;
    }
}
fn switch_note(mut store: AppStore, draft: Signal<Note>, next: Note) {
    let current = draft.peek().clone();
    let saved = store.state.peek().notes.iter().any(|n| n == &current);
    if current.title.trim().is_empty() && (!current.body.is_empty() || current.topic.is_some()) {
        store.error.set(
            "Give this draft a title before opening another note. Your text is still here.".into(),
        );
        return;
    }
    if saved || (current.title.is_empty() && current.body.is_empty()) {
        remember(store, draft, move |n| *n = next);
    } else {
        commit_then(
            store,
            move |s| {
                if let Some(n) = s.notes.iter_mut().find(|n| n.id == current.id) {
                    *n = current;
                } else {
                    s.notes.push(current);
                }
            },
            move || remember(store, draft, move |n| *n = next),
        );
    }
}
#[component]
fn Notes(mut store: AppStore, disabled: bool) -> Element {
    let draft = use_signal(|| {
        load_draft()
            .ok()
            .and_then(|x| x.as_string())
            .and_then(|x| serde_json::from_str::<Note>(&x).ok())
            .unwrap_or_else(new_note)
    });
    let mut generation = use_signal(|| 0_u64);
    use_effect(move || {
        let note = draft();
        let ready = *store.ready.read();
        let busy = *store.busy.peek();
        let same = store.state.read().notes.iter().any(|n| n == &note);
        let token = generation.peek().wrapping_add(1);
        generation.set(token);
        if !ready || busy || same || note.title.trim().is_empty() {
            return;
        }
        spawn(async move {
            gloo_timers::future::TimeoutFuture::new(600).await;
            if *generation.peek() != token {
                return;
            }
            commit(store, move |s| {
                if let Some(n) = s.notes.iter_mut().find(|n| n.id == note.id) {
                    *n = note;
                } else {
                    s.notes.push(note);
                }
            });
        });
    });
    let notes = store.state.read().notes.clone();
    let current = draft();
    rsx! {
        PageTitle { kicker: "06 / THE MARGINS", title: "Think on paper.", subtitle: "Plain text notes with autosave. A recovery draft protects edits during refresh; save errors stay visible." }
        div { class: "notes-grid",
            section { class: "panel note-index",
                div { class: "panel-heading", h2 { "Notebook" }
                    button { class: "text-button", onclick: move |_| { switch_note(store,draft,new_note()); }, "+ NEW" }
                }
                if notes.is_empty() { Empty { title: "No notes yet.", body: "Keep a derivation, a question, a moment of understanding." } }
                for n in notes {
                    button { class: if current.id==n.id { "note-link selected" } else { "note-link" },
                        onclick: move |_| { switch_note(store,draft,n.clone()); },
                        strong { "{n.title}" } small { {n.topic.clone().unwrap_or_else(||"Unlinked note".into())} }
                    }
                }
            }
            section { class: "panel editor",
                form { onsubmit: move |e| { e.prevent_default(); let note=draft(); commit(store,move |s|{if let Some(n)=s.notes.iter_mut().find(|n|n.id==note.id){*n=note;}else{s.notes.push(note);}}); },
                    label { "Note title" input { required:true, maxlength:500, placeholder:"An idea worth keeping", value:"{current.title}", oninput:move |e| remember(store,draft,move |n|n.title=e.value()) } }
                    label { "Chapter / topic label" input { value:current.topic.clone().unwrap_or_default(), placeholder:"e.g. Kinematics", oninput:move |e|remember(store,draft,move |n|n.topic=if e.value().trim().is_empty(){None}else{Some(e.value())}) } }
                    label { "Your notes" textarea { class:"note-paper", placeholder:"Start with what you understand. Leave room for what you don't.", value:"{current.body}", oninput:move |e|remember(store,draft,move |n|n.body=e.value()) } }
                    button { class:"primary", disabled, "Save note" span { "↗" } }
                }
            }
        }
    }
}
#[component]
fn Practice(store: AppStore, disabled: bool) -> Element {
    let mut topic = use_signal(String::new);
    let mut outcome = use_signal(|| "Independent".to_string());
    let mut seconds = use_signal(|| "180".to_string());
    let mut mistake = use_signal(String::new);
    let attempts = store.state.read().attempts.clone();
    rsx! {
        PageTitle {
            kicker: "07 / UNDERSTANDING THROUGH DOING",
            title: "Work through it.",
            subtitle: "Record your own question attempts. Hinted solutions never count as independently correct.",
        }
        div { class: "work-grid",
            section { class: "panel",
                h2 { "Attempt log" }
                if attempts.is_empty() {
                    Empty {
                        title: "Your first honest attempt.",
                        body: "You don't need to get it right. You need to know what happened.",
                    }
                }
                for a in attempts.iter().rev().take(50) {
                    div { class: "attempt-row",
                        div {
                            h3 { "{a.topic}" }
                            p { "{a.date} / {a.seconds} SECONDS" }
                        }
                        span { class: "tag", "{a.outcome:?}" }
                        if !a.mistake.is_empty() {
                            p { class: "mistake-text", "{a.mistake}" }
                        }
                    }
                }
            }
            section { class: "panel form-panel",
                h2 { "Record an attempt" }
                form {
                    onsubmit: move |e| {
                        e.prevent_default();
                        let a = Attempt {
                            id: uid(),
                            topic: topic().trim().into(),
                            outcome: match outcome().as_str() {
                                "Hinted" => Outcome::Hinted,
                                "Incorrect" => Outcome::Incorrect,
                                "Skipped" => Outcome::Skipped,
                                _ => Outcome::Independent,
                            },
                            seconds: seconds().parse().unwrap_or(0),
                            date: today(),
                            mistake: mistake(),
                        };
                        commit(
                            store,
                            move |s| {
                                if !a.mistake.trim().is_empty() {
                                    s.cards
                                        .push(Card {
                                            id: uid(),
                                            front: format!("Revisit: {}", a.topic),
                                            back: a.mistake.clone(),
                                            due: a.date.clone(),
                                            stage: 0,
                                            reviews: 0,
                                        });
                                }
                                s.attempts.push(a)
                            },
                        );
                    },
                    label {
                        "Question / topic / source"
                        input {
                            required: true,
                            value: "{topic}",
                            oninput: move |e| topic.set(e.value()),
                            placeholder: "e.g. Kinematics · my textbook Q14",
                        }
                    }
                    label {
                        "Outcome"
                        select {
                            value: "{outcome}",
                            onchange: move |e| outcome.set(e.value()),
                            for name in ["Independent", "Hinted", "Incorrect", "Skipped"] {
                                option { value: "{name}", "{name}" }
                            }
                        }
                    }
                    label {
                        "Time taken (seconds)"
                        input {
                            r#type: "number",
                            min: 1,
                            max: 86400,
                            required: true,
                            value: "{seconds}",
                            oninput: move | e |
                                    seconds.set(e.value()),
                        }
                    }
                    label {
                        "Mistake / recall prompt (optional)"
                        textarea {
                            value: "{mistake}",
                            oninput: move |e| mistake.set(e.value()),
                            placeholder: "Adding a mistake also creates a revision card.",
                        }
                    }
                    button { class: "primary", disabled,
                        "Save attempt"
                        span { "+" }
                    }
                }
            }
        }
    }
}
#[component]
fn Revision(store: AppStore, disabled: bool) -> Element {
    let mut front = use_signal(String::new);
    let mut back = use_signal(String::new);
    let mut revealed = use_signal(|| None::<String>);
    let cards = store.state.read().cards.clone();
    let due: Vec<_> = cards.iter().filter(|c| c.due <= today()).collect();
    rsx! {
        PageTitle {
            kicker: "08 / KEEP WHAT YOU LEARN",
            title: "Return. Recall. Retain.",
            subtitle: "Review checkpoints: 1, 3, 7, 14, 30 days. A failed recall returns to one day.",
        }
        div { class: "work-grid",
            section { class: "panel",
                div { class: "panel-heading",
                    h2 { "Due for recall" }
                    span { class: "eyebrow", "{due.len()} DUE / {cards.len()} TOTAL" }
                }
                if due.is_empty() {
                    Empty {
                        title: "A little breathing room.",
                        body: "No cards are due. New cards are due immediately; future reviews appear on their scheduled date.",
                    }
                }
                for c in due {
                    div { class: "review-card",
                        span { class: "eyebrow", "{c.due} / {c.reviews} REVIEWS" }
                        h3 { "{c.front}" }
                        if revealed() == Some(c.id.clone()) {
                            p { class: "answer", "{c.back}" }
                            div { class: "button-row",
                                button {
                                    class: "secondary",
                                    disabled,
                                    onclick: {
                                        let id = c.id.clone();
                                        move |_| {
                                            let id = id.clone();
                                            commit(
                                                store,
                                                move |s| {
                                                    if let Some(c) = s.cards.iter_mut().find(|c| c.id == id) {
                                                        let _ = review(c, false, &today());
                                                    }
                                                },
                                            );
                                            revealed.set(None);
                                        }
                                    },
                                    "Again · 1 day"
                                }
                                button {
                                    class: "primary",
                                    disabled,
                                    onclick: {
                                        let id = c.id.clone();
                                        move |_| {
                                            let id = id.clone();
                                            commit(
                                                store,
                                                move |s| {
                                                    if let Some(c) = s.cards.iter_mut().find(|c| c.id == id) {
                                                        let _ = review(c, true, &today());
                                                    }
                                                },
                                            );
                                            revealed.set(None);
                                        }
                                    },
                                    "Recalled ✓"
                                }
                            }
                        } else {
                            button {
                                class: "secondary",
                                onclick: {
                                    let id = c.id.clone();
                                    move |_| revealed.set(Some(id.clone()))
                                },
                                "Reveal answer"
                            }
                        }
                    }
                }
            }
            section { class: "panel form-panel",
                h2 { "Make a recall card" }
                form {
                    onsubmit: move |e| {
                        e.prevent_default();
                        let c = Card {
                            id: uid(),
                            front: front().trim().into(),
                            back: back(),
                            due: today(),
                            stage: 0,
                            reviews: 0,
                        };
                        commit(store, move |s| s.cards.push(c));
                    },
                    label {
                        "Prompt"
                        textarea {
                            required: true,
                            value: "{front}",
                            oninput: move |e| front.set(e.value()),
                        }
                    }
                    label {
                        "Answer"
                        textarea {
                            value: "{back}",
                            oninput: move | e | back.set(e
                                    .value()),
                        }
                    }
                    button { class: "primary", disabled,
                        "Create card"
                        span { "+" }
                    }
                }
            }
        }
    }
}
#[component]
fn Analytics(store: AppStore) -> Element {
    let s = store.state.read();
    let acc = accuracy(&s.attempts)
        .map(|x| format!("{x:.1}%"))
        .unwrap_or("—".into());
    let done = s.tasks.iter().filter(|t| t.done).count();
    let reviews: u32 = s.cards.iter().map(|c| c.reviews).sum();
    rsx! {
        PageTitle {
            kicker: "11 / EVIDENCE, NOT ASSUMPTIONS",
            title: "See what is real.",
            subtitle: "Every number comes from your own records. No predicted ranks. No invented study hours.",
        }
        section { class: "metrics",
            div {
                span { "INDEPENDENT ACCURACY" }
                strong { "{acc}" }
                small { "Hints remain in the attempted denominator" }
            }
            div {
                span { "RECORDED ATTEMPTS" }
                strong { "{s.attempts.len()}" }
                small { "Includes explicitly skipped questions" }
            }
            div {
                span { "COMPLETED TASKS" }
                strong { "{done}" }
                small { "Completion does not imply mastery" }
            }
            div {
                span { "RECALL REVIEWS" }
                strong { "{reviews}" }
                small { "Cards created are not reviews completed" }
            }
        }
        div { class: "panel",
            h2 { "Build a history worth reading." }
            p { class: "muted",
                "Trends and weak-topic analysis need more context. Mock-test scoring and study-time analytics are not implemented in this build."
            }
        }
    }
}
#[component]
fn Settings(mut store: AppStore, disabled: bool) -> Element {
    let mut pending = use_signal(|| None::<State>);
    let mut confirmed = use_signal(|| false);
    rsx! {
        PageTitle {
            kicker: "12 / MAKE IT YOURS",
            title: "Your space. Your data.",
            subtitle: "Device-local IndexedDB storage. Export regularly; browser storage can be evicted.",
        }
        section { class: "panel",
            h2 { "Appearance" }
            div { class: "theme-options",
                for name in ["NOCTURNE", "MONO", "PAPER", "PLUM", "SKY"] {
                    button {
                        class: if store.state.read().theme == name { "theme-option selected" } else { "theme-option" },
                        disabled,
                        onclick: move | _ | commit(store, move | s | s.theme
                                = name.into()),
                        span { class: "theme-swatch swatch-{name}" }
                        "{name}"
                    }
                }
            }
            p { class: "muted small",
                "Motion follows your system's reduced-motion setting. Wallpaper import is not yet implemented."
            }
        }
        section { class: "panel backup-panel",
            h2 { "A copy you own." }
            p { "Portable JSON backup · schema version 1 · SHA-256 integrity verification" }
            p { class: "muted",
                "Includes tasks, notes, attempts, revision cards and preferences. No music tokens or external files are included."
            }
            div { class: "button-row",
                button {
                    class: "primary",
                    disabled,
                    onclick: move |_| {
                        match export(&store.state.read()) {
                            Ok(json) => download(&json, &format!("kuro-backup-{}.json", today())),
                            Err(e) => store.error.set(e),
                        }
                    },
                    "Export backup ↓"
                }
                button {
                    class: "secondary",
                    disabled,
                    onclick: move |_| {
                        spawn(async move {
                            match pick_backup().await {
                                Ok(raw) => {
                                    let text = raw.as_string().unwrap_or_default();
                                    if !text.is_empty() {
                                        match import(&text) {
                                            Ok(s) => {
                                                pending.set(Some(s));
                                                confirmed.set(false);
                                            }
                                            Err(e) => store.error.set(e),
                                        }
                                    }
                                }
                                Err(e) => store.error.set(describe(e)),
                            }
                        });
                    },
                    "Choose backup to restore"
                }
            }
            if let Some(s) = pending() {
                div { class: "restore-preview",
                    h3 { "Review restore" }
                    p {
                        "{s.tasks.len()} tasks · {s.notes.len()} notes · {s.attempts.len()} attempts · {s.cards.len()} cards"
                    }
                    p {
                        "Restoring replaces current study records. Export a copy of your current data first."
                    }
                    label { class: "checkbox-label",
                        input {
                            r#type: "checkbox",
                            checked: confirmed(),
                            onchange: move |e| confirmed.set(e.checked()),
                        }
                        "I have exported my current data and want to replace it."
                    }
                    div { class: "button-row",
                        button {
                            class: "primary",
                            disabled: disabled || !confirmed(),
                            onclick: move |_| {
                                if let Some(incoming) = pending() {
                                    commit_then(
                                        store,
                                        move |s| {
                                            let rev = s.revision;
                                            *s = incoming;
                                            s.revision = rev;
                                        },
                                        move || { if let Err(e)=clear_draft() {store.error.set(format!("Data restored, but recovery draft cleanup failed: {}",describe(e)));} },
                                    );
                                    pending.set(None);
                                }
                            },
                            "Restore verified backup"
                        }
                        button {
                            class: "secondary",
                            onclick: move | _
                                    | pending.set(None),
                            "Cancel"
                        }
                    }
                }
            }
        }
        section { class: "panel",
            h2 { "Storage and privacy" }
            p { class: "muted",
                "Writes are transactional. Failed writes leave previous data intact and display an error. Conflicting edits from another tab are rejected; reload before retrying. There is no server sync in this build."
            }
            p { class: "eyebrow", "STORED REVISION / {store.state.read().revision}" }
        }
    }
}
#[component]
fn Music() -> Element {
    let mut provider = use_signal(|| "Apple Music".to_string());
    rsx! {
        PageTitle {
            kicker: "10 / A ROOM WITH A RECORD",
            title: "Find your frequency.",
            subtitle: "An analog-inspired space. Streaming integration is awaiting provider configuration and implementation.",
        }
        section { class: "music-room",
            div { class: "turntable",
                div { class: "vinyl",
                    div { class: "record-label",
                        span { "KURO" }
                        small { "SIDE A / SILENCE" }
                    }
                }
                div { class: "tonearm" }
            }
            div { class: "music-info",
                span { class: "eyebrow", "PROVIDER / NOT CONNECTED" }
                h2 { "The quiet\nbefore the music." }
                p {
                    "No track is playing. Artwork and controls will appear only when a supported provider reports real playback."
                }
                label {
                    "Music provider"
                    select {
                        value: "{provider}",
                        onchange: move | e |
                                provider.set(e.value()),
                        option { "Apple Music" }
                        option { "Spotify" }
                    }
                }
                p { class: "muted small",
                    if provider() == "Spotify" {
                        "Requires a registered Spotify app, PKCE authorization and an eligible Premium account for SDK playback. No client secret belongs in this website."
                    } else {
                        "Requires MusicKit authorization and a securely issued developer token. Apple private keys must remain on a trusted server."
                    }
                }
                span { class: "tag", "CONFIGURATION REQUIRED · PLAYBACK UNAVAILABLE" }
            }
        }
    }
}
#[component]
fn Unavailable(name: String) -> Element {
    rsx! {
        PageTitle {
            kicker: "PLANNED WORKSPACE",
            title: name.clone(),
            subtitle: "This module has not been implemented. No data or completed functionality is implied.",
        }
        div { class: "panel",
            h2 { "Module status" }
            Empty {
                title: format!("{} is still on the workbench.", name),
                body: "Tasks, notes, practice, revision and backups work locally in this foundation. PDF rendering, annotations, exam scoring and native applications require their own verified milestones.",
            }
        }
    }
}
#[component]
fn Empty(title: String, body: String) -> Element {
    rsx! {
        div { class: "empty-state",
            span { class: "empty-symbol", "aria-hidden": "true", "＋" }
            h3 { "{title}" }
            p { "{body}" }
        }
    }
}
