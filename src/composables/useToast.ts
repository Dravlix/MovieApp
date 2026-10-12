import { ref } from 'vue';

export interface Toast {
  id: string;
  type: 'success' | 'info' | 'warning' | 'error';
  title?: string;
  message: string;
  duration?: number;
}

const toasts = ref<Toast[]>([]);

export function useToast() {
  const show = (toast: Omit<Toast, 'id'>) => {
    const id = Math.random().toString(36).substring(2, 9);
    const newToast: Toast = {
      id,
      duration: 3500,
      ...toast,
    };
    toasts.value.push(newToast);

    if (newToast.duration && newToast.duration > 0) {
      setTimeout(() => {
        remove(id);
      }, newToast.duration);
    }
  };

  const remove = (id: string) => {
    toasts.value = toasts.value.filter((t) => t.id !== id);
  };

  const success = (message: string, title: string = 'Úspěch') => {
    show({ type: 'success', title, message });
  };

  const info = (message: string, title?: string) => {
    show({ type: 'info', title, message });
  };

  const warning = (message: string, title: string = 'Upozornění') => {
    show({ type: 'warning', title, message });
  };

  const error = (message: string, title: string = 'Chyba') => {
    show({ type: 'error', title, message });
  };

  return {
    toasts,
    show,
    remove,
    success,
    info,
    warning,
    error,
  };
}
