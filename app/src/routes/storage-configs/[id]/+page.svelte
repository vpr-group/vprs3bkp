<script lang="ts">
  import { onMount } from "svelte";
  import { StoreService, type StorageConfig } from "../../../services/store";
  import { page } from "$app/state";
  import StorageProviderDialog from "../../../components/StorageConfigDialog.svelte";
  import { ActionsService, type Entry } from "../../../services/actions";
  import Table, { type Cell, type Row } from "../../../components/Table.svelte";
  import Separation from "../../../components/Separation.svelte";
  import Button from "../../../components/Button.svelte";
  import RestoreDropdown from "../../../components/RestoreDropdown.svelte";
  import { notificationsStore } from "../../../components/Notifications.svelte";
  import { goto } from "$app/navigation";

  const { addNotification, removeNotification } = notificationsStore;
  const storeService = new StoreService();
  const actionsService = new ActionsService();

  let storageConfig = $state<StorageConfig | null>(null);
  let backups = $state<Entry[]>([]);

  const loadStorageConfig = async () => {
    await storeService.waitForInitialized();
    storageConfig = await storeService.getStorageConfig(page.params.id);
  };

  const loadBackups = async () => {
    if (!storageConfig) return;

    try {
      backups = await actionsService.list(storageConfig);
    } catch (error) {
      addNotification({
        title: "Failed to load backups",
        message: `${error}`,
        status: "error",
      });
    }
  };

  onMount(async () => {
    loadStorageConfig();
  });

  $effect(() => {
    loadBackups();
  });
</script>

{#snippet sideSection()}
  {#if storageConfig}
    <Button
      icon="cross"
      onclick={async () => {
        if (!storageConfig) return;
        await storeService.deleteStorageConfig(storageConfig.id);
        goto("/");
      }}>Delete</Button
    >

    <StorageProviderDialog
      {storageConfig}
      onsubmit={async (storageProvider) => {
        await storeService.saveStorageConfig(storageProvider);
        loadStorageConfig();
      }}
    />
    <Button onclick={() => loadBackups()} icon="reload">Refresh</Button>
  {/if}
{/snippet}

{#if storageConfig}
  <Separation
    label={storageConfig.name}
    subLabel={`${storageConfig.type} - ${storageConfig.name}`}
    {sideSection}
  />

  {#snippet actions(cell: Cell, row?: Row)}
    <RestoreDropdown
      backupKey={cell.label || ""}
      onrestore={async ({ sourceConfig, dropDatabase }) => {
        if (!storageConfig) return;

        const progressNotifications = addNotification({
          title: "Restore in progress...",
          status: "info",
          dismissTimeout: null,
        });

        try {
          await actionsService.restore(
            cell.label || "",
            sourceConfig,
            storageConfig,
            dropDatabase
          );

          removeNotification(progressNotifications.id);
          addNotification({
            title: "Restore successful",
            status: "success",
          });
        } catch (error) {
          removeNotification(progressNotifications.id);
          addNotification({
            title: "Failed to restore",
            message: `${error}`,
            status: "error",
          });
        }
      }}
    />
  {/snippet}

  <Table
    headers={[
      { label: "#", width: "3rem" },
      { label: "key", width: "40%" },
      // { label: "type", width: "10%" },
      // { label: "Timestamp" },
    ]}
    rows={backups.map((row, index) => ({
      cells: [
        { label: (index + 1).toString().padStart(2, "0") },
        { label: row.path },
        {
          label: row.path,
          renderHandler: actions,
          style: {
            padding: "0 0.5rem",
            alignItems: "center",
            justifyContent: "flex-end",
            flex: "1 1 auto",
            width: "100%",
          },
        },
      ],
    }))}
  />
{/if}
