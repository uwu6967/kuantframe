import { TauriTypes } from "$types";
import api, { SendTauriEvent } from "@api/index";
import { useTranslateCommon, useTranslateForms } from "@hooks/useTranslate.hook";
import { Box, Button, Group, Select, Stack, Text } from "@mantine/core";
import { UseFormReturnType } from "@mantine/form";
import { modals } from "@mantine/modals";
import { notifications } from "@mantine/notifications";
import { relaunch } from "@tauri-apps/plugin-process";
import { useEffect, useState } from "react";
export type GeneralPanelProps = {
  form: UseFormReturnType<TauriTypes.Settings>;
};

const languages = [
  { label: "German", value: "de" },
  { label: "English", value: "en" },
  { label: "Spanish", value: "es" },
  { label: "French", value: "fr" },
  { label: "Italian", value: "it" },
  { label: "Korean", value: "ko" },
  { label: "Polish", value: "pl" },
  { label: "Portuguese (Brazil)", value: "pt" },
  { label: "Russian", value: "ru" },
  { label: "Ukrainian", value: "uk" },
  { label: "Chinese (Simplified)", value: "zh" },
  { label: "Chinese (Traditional)", value: "tc" },
  { label: "Japanese", value: "ja" },
  { label: "Thai", value: "th" },
  { label: "Turkish", value: "tr" },
];

function invokeErrorMessage(error: unknown): string {
  if (error && typeof error === "object" && "message" in error) {
    const message = (error as { message: unknown }).message;
    if (typeof message === "string" && message.trim().length > 0) {
      return message;
    }
  }
  return String(error);
}

export const GeneralPanel = ({ form }: GeneralPanelProps) => {
  const [defaultSettings, setDefaultSettings] = useState<TauriTypes.Settings | null>(null);
  const [importing, setImporting] = useState(false);

  const useTranslateForm = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateForms(`settings.tabs.general.${key}`, { ...context }, i18Key);
  const useTranslateFormFields = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateForm(`fields.${key}`, { ...context }, i18Key);
  const useTranslateFormButtons = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateForm(`buttons.${key}`, { ...context }, i18Key);
  const useTranslateFormPrompt = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateForm(`prompt.${key}`, { ...context }, i18Key);

  useEffect(() => {
    api.app.getDefaultSettings().then(setDefaultSettings).catch(console.error);
  }, []);

  const isDefaultSettings = () => {
    return JSON.stringify(form.values) != JSON.stringify(defaultSettings);
  };

  const handleReset = () => {
    modals.openConfirmModal({
      title: useTranslateFormPrompt("reset_settings.title"),
      children: <Text size="sm">{useTranslateFormPrompt("reset_settings.message")}</Text>,
      labels: { confirm: useTranslateFormButtons("prompt_reset_label"), cancel: useTranslateCommon("buttons.cancel.label") },
      confirmProps: { color: "red" },
      onConfirm: async () => {
        const defaults = await api.app.getDefaultSettings();
        await api.app.updateSettings(defaults);
        modals.closeAll();
        SendTauriEvent(TauriTypes.Events.RefreshSettings);
      },
    });
  };

  const handleImportQuantframe = () => {
    modals.openConfirmModal({
      title: useTranslateFormPrompt("import_quantframe.title"),
      children: <Text size="sm">{useTranslateFormPrompt("import_quantframe.message")}</Text>,
      labels: { confirm: useTranslateFormPrompt("import_quantframe.confirm"), cancel: useTranslateCommon("buttons.cancel.label") },
      confirmProps: { color: "red" },
      onConfirm: async () => {
        setImporting(true);
        try {
          await api.app.importQuantframeSave();
          notifications.show({
            title: useTranslateForm("import_quantframe.success_title"),
            message: useTranslateForm("import_quantframe.success_message"),
            color: "green.7",
          });
          await relaunch();
        } catch (error) {
          notifications.show({
            title: useTranslateForm("import_quantframe.error_title"),
            message: invokeErrorMessage(error),
            color: "red.7",
          });
        } finally {
          setImporting(false);
        }
      },
    });
  };

  return (
    <Box h="100%" p={"md"}>
      <Stack>
        <Group gap="md">
          <Select
            allowDeselect={false}
            w={150}
            label={useTranslateFormFields("language.label")}
            placeholder={useTranslateFormFields("language.placeholder")}
            data={languages}
            {...form.getInputProps("lang")}
            radius="md"
          />
        </Group>
        <Stack gap="xs" maw={520}>
          <Text size="sm" c="dimmed">
            {useTranslateForm("import_quantframe.description")}
          </Text>
          <Button onClick={handleImportQuantframe} loading={importing} color="orange.7" w="fit-content">
            {useTranslateFormButtons("import_quantframe_label")}
          </Button>
        </Stack>
        {isDefaultSettings() && (
          <Button onClick={handleReset} color="red.7" pos={"absolute"} bottom={55} right={45}>
            {useTranslateFormButtons("reset_settings_label")}
          </Button>
        )}
      </Stack>
    </Box>
  );
};
