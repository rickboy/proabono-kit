/*! proabono-kit runtime | MIT OR Apache-2.0 | https://github.com/rickboy/proabono-kit */
// Progressive enhancement for server-rendered proabono components. Every
// component works without this file (native popover, <dialog>, datalist);
// the runtime adds keyboard support, positioning fallbacks and toasts.
(() => {
  "use strict";
  if (window.proabono) return;

  const supportsAnchor = CSS.supports("anchor-name: --pb");
  const supportsCommand = "command" in HTMLButtonElement.prototype;
  const supportsClosedBy = "closedBy" in HTMLDialogElement.prototype;

  // ---------------------------------------------------------------------------
  // Dropdown menu (WAI-ARIA menu button pattern)
  // ---------------------------------------------------------------------------

  const menuItems = (menu) =>
    [...menu.querySelectorAll('[role="menuitem"]')].filter(
      (el) => el.getAttribute("aria-disabled") !== "true",
    );

  const focusAt = (items, index) => {
    if (!items.length) return;
    items[(index + items.length) % items.length].focus();
  };

  const triggerFor = (menu) => document.getElementById(menu.getAttribute("aria-labelledby"));

  // Only used where CSS anchor positioning is unavailable.
  function placeMenu(menu, trigger) {
    if (supportsAnchor || !trigger) return;
    const t = trigger.getBoundingClientRect();
    menu.style.minWidth = `${Math.max(t.width, 192)}px`;
    const m = menu.getBoundingClientRect();
    const align = menu.dataset.align || "start";
    let left = align === "end" ? t.right - m.width : align === "center" ? t.left + (t.width - m.width) / 2 : t.left;
    left = Math.max(8, Math.min(left, innerWidth - m.width - 8));
    let top = t.bottom + 4;
    if (top + m.height > innerHeight - 8 && t.top - m.height - 4 > 8) top = t.top - m.height - 4;
    Object.assign(menu.style, { position: "fixed", left: `${left}px`, top: `${top}px` });
  }

  document.addEventListener(
    "beforetoggle",
    (e) => {
      const menu = e.target;
      if (!(menu instanceof HTMLElement) || !menu.matches("[data-pb-menu-content]")) return;
      if (e.newState === "closed" && menu.contains(document.activeElement)) {
        // Closed from inside (Escape, Tab, selection): hand focus back.
        queueMicrotask(() => triggerFor(menu)?.focus());
      }
    },
    true,
  );

  document.addEventListener(
    "toggle",
    (e) => {
      const menu = e.target;
      if (!(menu instanceof HTMLElement) || !menu.matches("[data-pb-menu-content]")) return;
      const trigger = triggerFor(menu);
      const open = e.newState === "open";
      trigger?.setAttribute("aria-expanded", String(open));
      if (!open) return;
      placeMenu(menu, trigger);
      const items = menuItems(menu);
      focusAt(items, menu.dataset.pbFocusLast ? -1 : 0);
      delete menu.dataset.pbFocusLast;
    },
    true,
  );

  let typeahead = "";
  let typeaheadTimer = 0;

  function onMenuKeydown(e, menu) {
    const items = menuItems(menu);
    const index = items.indexOf(document.activeElement);
    switch (e.key) {
      case "ArrowDown":
        e.preventDefault();
        focusAt(items, index + 1);
        return;
      case "ArrowUp":
        e.preventDefault();
        focusAt(items, index - 1);
        return;
      case "Home":
        e.preventDefault();
        focusAt(items, 0);
        return;
      case "End":
        e.preventDefault();
        focusAt(items, -1);
        return;
      case "Tab":
        menu.hidePopover();
        return;
      case " ":
        if (document.activeElement instanceof HTMLAnchorElement) {
          e.preventDefault();
          document.activeElement.click();
        }
        return;
    }
    if (e.key.length === 1 && !e.ctrlKey && !e.metaKey && !e.altKey) {
      clearTimeout(typeaheadTimer);
      typeahead += e.key.toLowerCase();
      typeaheadTimer = setTimeout(() => (typeahead = ""), 500);
      const start = typeahead.length === 1 ? index + 1 : index;
      const ordered = [...items.slice(start), ...items.slice(0, start)];
      ordered.find((el) => el.textContent.trim().toLowerCase().startsWith(typeahead))?.focus();
    }
  }

  document.addEventListener("keydown", (e) => {
    const target = e.target;
    if (!(target instanceof Element)) return;

    const trigger = target.closest("[data-pb-menu-trigger]");
    if (trigger && (e.key === "ArrowDown" || e.key === "ArrowUp")) {
      const menu = document.getElementById(trigger.getAttribute("aria-controls"));
      if (menu && !menu.matches(":popover-open")) {
        e.preventDefault();
        if (e.key === "ArrowUp") menu.dataset.pbFocusLast = "1";
        menu.showPopover();
      }
      return;
    }

    const menu = target.closest("[data-pb-menu-content]");
    if (menu) onMenuKeydown(e, menu);
  });

  document.addEventListener("pointermove", (e) => {
    const item = e.target instanceof Element && e.target.closest("[data-pb-menu-content] [role='menuitem']");
    if (item && item !== document.activeElement && item.getAttribute("aria-disabled") !== "true") item.focus();
  });

  document.addEventListener("click", (e) => {
    const item = e.target instanceof Element && e.target.closest("[data-pb-menu-content] [role='menuitem']");
    if (!item || item.getAttribute("aria-disabled") === "true") return;
    const menu = item.closest("[data-pb-menu-content]");
    if (item.dataset.value !== undefined) {
      // Dispatched from the menu element, so `event.target.id` is the menu id.
      menu.dispatchEvent(
        new CustomEvent("proabono:select", {
          bubbles: true,
          detail: { value: item.dataset.value, label: item.textContent.trim() },
        }),
      );
    }
    menu.hidePopover();
  });

  // ---------------------------------------------------------------------------
  // Combobox (WAI-ARIA combobox with list autocomplete)
  // ---------------------------------------------------------------------------

  function initCombobox(root) {
    if (root.dataset.pbReady) return;
    root.dataset.pbReady = "";

    const input = root.querySelector("[data-pb-combobox-input]");
    const hidden = root.querySelector("[data-pb-combobox-value]");
    const listbox = root.querySelector("[role='listbox']");
    const empty = root.querySelector("[data-pb-combobox-empty]");
    const options = [...listbox.querySelectorAll("[role='option']")];
    let active = -1;

    // Swap the native fallback (text input + datalist) for the ARIA widget.
    hidden.name = input.name;
    input.removeAttribute("name");
    input.removeAttribute("list");
    root.querySelector("datalist")?.remove();
    input.setAttribute("role", "combobox");
    input.setAttribute("aria-autocomplete", "list");
    input.setAttribute("aria-expanded", "false");

    const visible = () => options.filter((o) => !o.hidden);
    const isOpen = () => !listbox.hidden;

    const setActive = (i) => {
      const vis = visible();
      options.forEach((o) => o.removeAttribute("data-active"));
      active = i;
      const option = vis[i];
      if (option) {
        option.setAttribute("data-active", "");
        input.setAttribute("aria-activedescendant", option.id);
        option.scrollIntoView({ block: "nearest" });
      } else {
        input.removeAttribute("aria-activedescendant");
      }
    };

    const filter = (query) => {
      const q = query.trim().toLowerCase();
      let count = 0;
      for (const o of options) {
        const show = !q || o.dataset.label.toLowerCase().includes(q);
        o.hidden = !show;
        if (show) count++;
      }
      if (empty) empty.hidden = count > 0;
      const selected = visible().findIndex((o) => o.getAttribute("aria-selected") === "true");
      setActive(count ? Math.max(selected, 0) : -1);
    };

    const open = (query = "") => {
      filter(query);
      listbox.hidden = false;
      input.setAttribute("aria-expanded", "true");
    };

    const close = () => {
      listbox.hidden = true;
      input.setAttribute("aria-expanded", "false");
      setActive(-1);
    };

    const select = (option) => {
      hidden.value = option.dataset.value;
      input.value = option.dataset.label;
      options.forEach((o) => o.setAttribute("aria-selected", String(o === option)));
      close();
      hidden.dispatchEvent(new Event("change", { bubbles: true }));
      root.dispatchEvent(
        new CustomEvent("proabono:change", {
          bubbles: true,
          detail: { value: option.dataset.value, label: option.dataset.label },
        }),
      );
    };

    input.addEventListener("input", () => {
      hidden.value = "";
      options.forEach((o) => o.setAttribute("aria-selected", "false"));
      open(input.value);
    });

    input.addEventListener("click", () => {
      if (!isOpen()) open();
    });

    input.addEventListener("keydown", (e) => {
      const vis = visible();
      switch (e.key) {
        case "ArrowDown":
          e.preventDefault();
          if (!isOpen()) open();
          else setActive(Math.min(active + 1, vis.length - 1));
          break;
        case "ArrowUp":
          e.preventDefault();
          if (isOpen()) setActive(Math.max(active - 1, 0));
          break;
        case "Enter":
          if (isOpen() && vis[active]) {
            e.preventDefault();
            select(vis[active]);
          }
          break;
        case "Escape":
          if (isOpen()) {
            e.preventDefault();
            close();
          } else if (input.value) {
            input.value = "";
            hidden.value = "";
            options.forEach((o) => o.setAttribute("aria-selected", "false"));
          }
          break;
      }
    });

    input.addEventListener("blur", () => {
      close();
      if (hidden.value) return;
      const typed = input.value.trim().toLowerCase();
      const match = options.find(
        (o) => o.dataset.label.toLowerCase() === typed || o.dataset.value.toLowerCase() === typed,
      );
      if (match) select(match);
      else if (root.hasAttribute("data-strict")) input.value = "";
    });

    // Keep focus in the input while clicking options.
    listbox.addEventListener("pointerdown", (e) => e.preventDefault());
    listbox.addEventListener("click", (e) => {
      const option = e.target instanceof Element && e.target.closest("[role='option']");
      if (option) select(option);
    });
  }

  // ---------------------------------------------------------------------------
  // Drawer (native <dialog>; fills gaps in invoker commands / closedby)
  // ---------------------------------------------------------------------------

  document.addEventListener("click", (e) => {
    const target = e.target;
    if (!(target instanceof Element)) return;

    const opener = target.closest("[data-pb-drawer-open]");
    if (opener && !supportsCommand) {
      document.getElementById(opener.getAttribute("commandfor"))?.showModal();
      return;
    }

    if (!supportsClosedBy && target instanceof HTMLDialogElement && target.matches("[data-pb-drawer][open]")) {
      const r = target.getBoundingClientRect();
      const outside = e.clientX < r.left || e.clientX > r.right || e.clientY < r.top || e.clientY > r.bottom;
      if (outside) target.close();
    }
  });

  // ---------------------------------------------------------------------------
  // Toast (Sonner-style stack)
  // ---------------------------------------------------------------------------

  const VISIBLE_TOASTS = 3;
  const ICONS = {
    default: '<path d="M12 8v4m0 4h.01"/><circle cx="12" cy="12" r="10"/>',
    info: '<circle cx="12" cy="12" r="10"/><path d="M12 16v-4m0-4h.01"/>',
    success: '<circle cx="12" cy="12" r="10"/><path d="m9 12 2 2 4-4"/>',
    warning: '<path d="m21.73 18-8-14a2 2 0 0 0-3.48 0l-8 14A2 2 0 0 0 4 21h16a2 2 0 0 0 1.73-3"/><path d="M12 9v4m0 4h.01"/>',
    error: '<circle cx="12" cy="12" r="10"/><path d="m15 9-6 6m0-6 6 6"/>',
  };
  const svg = (paths) =>
    `<svg class="pb-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true">${paths}</svg>`;

  const timers = new WeakMap();

  function layout(toaster) {
    const list = toaster.querySelector(".pb-toast-list");
    const live = [...list.children].filter((li) => !li.hasAttribute("data-removing")).reverse();
    let offset = 0;
    live.forEach((li, i) => {
      li.style.setProperty("--i", String(i));
      li.style.setProperty("--offset", `${offset}px`);
      li.toggleAttribute("data-front", i === 0);
      li.toggleAttribute("data-overflow", i >= VISIBLE_TOASTS);
      li.style.height = "";
      offset += li.offsetHeight + 12;
    });
    if (live[0]) toaster.style.setProperty("--pb-front-height", `${live[0].offsetHeight}px`);
    list.style.height = live[0] ? `${live[0].offsetHeight}px` : "0px";
  }

  function schedule(toast) {
    const duration = Number(toast.dataset.duration || 4000);
    if (!duration) return;
    timers.set(toast, { remaining: duration, start: Date.now(), id: setTimeout(() => dismiss(toast), duration) });
  }

  function pause(toaster) {
    toaster.querySelectorAll(".pb-toast").forEach((t) => {
      const timer = timers.get(t);
      if (!timer || timer.paused) return;
      clearTimeout(timer.id);
      timer.remaining -= Date.now() - timer.start;
      timer.paused = true;
    });
  }

  function resume(toaster) {
    toaster.querySelectorAll(".pb-toast").forEach((t) => {
      const timer = timers.get(t);
      if (!timer || !timer.paused) return;
      timer.paused = false;
      timer.start = Date.now();
      timer.id = setTimeout(() => dismiss(t), Math.max(timer.remaining, 1000));
    });
  }

  function dismiss(toast) {
    if (!toast.isConnected || toast.hasAttribute("data-removing")) return;
    clearTimeout(timers.get(toast)?.id);
    const toaster = toast.closest("[data-pb-toaster]");
    toast.setAttribute("data-removing", "");
    setTimeout(() => {
      toast.remove();
      if (toaster) {
        if (!toaster.querySelector(".pb-toast")) toaster.removeAttribute("data-expanded");
        layout(toaster);
      }
    }, 300);
    if (toaster) layout(toaster);
  }

  function ensureToaster() {
    let toaster = document.querySelector("[data-pb-toaster]");
    if (toaster) return toaster;
    toaster = document.createElement("section");
    toaster.className = "pb-toaster";
    toaster.dataset.pbToaster = "";
    toaster.dataset.position = "bottom-right";
    toaster.setAttribute("aria-label", "Notifications");
    toaster.innerHTML = '<ol class="pb-toast-list" aria-live="polite"></ol>';
    document.body.append(toaster);
    bindToaster(toaster);
    return toaster;
  }

  function bindToaster(toaster) {
    if (toaster.dataset.pbReady !== undefined) return;
    toaster.dataset.pbReady = "";
    const expand = () => {
      toaster.setAttribute("data-expanded", "");
      pause(toaster);
    };
    const collapse = () => {
      if (toaster.matches(":hover") || toaster.contains(document.activeElement)) return;
      toaster.removeAttribute("data-expanded");
      resume(toaster);
    };
    toaster.addEventListener("pointerenter", expand);
    toaster.addEventListener("pointerleave", collapse);
    toaster.addEventListener("focusin", expand);
    toaster.addEventListener("focusout", () => setTimeout(collapse));
    toaster.querySelectorAll(".pb-toast").forEach(schedule);
    layout(toaster);
  }

  function toast(options) {
    const opts = typeof options === "string" ? { title: options } : options || {};
    const tone = ICONS[opts.tone] ? opts.tone : "default";
    const toaster = ensureToaster();
    const li = document.createElement("li");
    li.className = "pb-toast";
    li.dataset.tone = tone;
    li.dataset.duration = String(opts.duration ?? 4000);
    li.setAttribute("role", tone === "error" ? "alert" : "status");

    const icon = document.createElement("span");
    icon.className = "pb-toast-icon";
    icon.innerHTML = svg(ICONS[tone]);

    const body = document.createElement("div");
    const title = document.createElement("p");
    title.className = "pb-toast-title";
    title.textContent = opts.title ?? "";
    body.append(title);
    if (opts.description) {
      const description = document.createElement("p");
      description.className = "pb-toast-description";
      description.textContent = opts.description;
      body.append(description);
    }

    const close = document.createElement("button");
    close.type = "button";
    close.className = "pb-icon-button";
    close.dataset.pbToastClose = "";
    close.setAttribute("aria-label", "Dismiss notification");
    close.innerHTML = svg('<path d="M18 6 6 18M6 6l12 12"/>');

    li.append(icon, body, close);
    toaster.querySelector(".pb-toast-list").append(li);
    layout(toaster);
    if (toaster.hasAttribute("data-expanded")) timers.set(li, { remaining: Number(li.dataset.duration), paused: true });
    else schedule(li);
    return { dismiss: () => dismiss(li) };
  }

  document.addEventListener("click", (e) => {
    const target = e.target;
    if (!(target instanceof Element)) return;
    const close = target.closest("[data-pb-toast-close]");
    if (close) {
      dismiss(close.closest(".pb-toast"));
      return;
    }
    const trigger = target.closest("[data-pb-toast]");
    if (trigger) {
      toast({
        title: trigger.dataset.pbToast,
        description: trigger.dataset.pbToastDescription,
        tone: trigger.dataset.pbToastTone,
        duration: trigger.dataset.pbToastDuration ? Number(trigger.dataset.pbToastDuration) : undefined,
      });
    }
  });

  // ---------------------------------------------------------------------------
  // Init
  // ---------------------------------------------------------------------------

  function init(root = document) {
    root.querySelectorAll("[data-pb-combobox]").forEach(initCombobox);
    root.querySelectorAll("[data-pb-toaster]").forEach(bindToaster);
  }

  new MutationObserver((records) => {
    for (const record of records) {
      for (const node of record.addedNodes) {
        if (node instanceof Element) {
          if (node.matches("[data-pb-combobox]")) initCombobox(node);
          if (node.matches("[data-pb-toaster]")) bindToaster(node);
          init(node);
        }
      }
    }
  }).observe(document.documentElement, { childList: true, subtree: true });

  window.proabono = { toast, init };
  if (document.readyState === "loading") document.addEventListener("DOMContentLoaded", () => init());
  else init();
})();
