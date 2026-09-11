import { TauriTypes, UserStatus } from "$types";

/** Bundled Catppuccin Mocha preset by NakedTrashPanda (GPL-friendly community theme). */
export const catppuccinMochaTheme = {
  name: "Catppuccin Mocha",
  author: "NakedTrashPanda",
  iconBase64: "iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAAAXNSR0IArs4c6QAAAARnQU1BAACxjwv8YQUAAAAJcEhZcwAADsMAAA7DAcdvqGQAAAQWSURBVHhe7Zu/a1RBEMfzl6RQEESIRRBRQRQRSaEiapNCRSRFQFIEEWKRKoI/QggaMBFUguSS3GnuciGICglIFLGzsbOys7G7yzUr34OVc/YuOzO7793Be8WneZ67O5/dtzO7d+nr7x8wWaaPPsgauQD6IGvkAuiDrJELoA+yRtcEnDkxbIZO3fgP+pk0SEXAyaOXzZ2bT0xx+rv5tlTzUp79YSZHX6YiJTEBg4fPmamxJbM599MJUML2i9/m4XixKZH2EYPoAg4eON6cvc+Lf5xgQpm5u24GDp12+gwhqoCxa4+bM0YHHhOInRiZN/v3HXH61xBFAGZ9YXLLGWySYD+JsRqCBeBd525usfkw/6uZTeiYJAQJwMaU9JL3gVdi+Py4MzYuagGY+W4Hb4EEbcpUCcA7j1xNB9JNMBnHBi84Y/WhEpD2hscFNQcmh453L8QCkOpox73E3L13zpj3QiQAaSeJAic2kqpRJAAVHu2sF5GsArYAbDAxZ397uWber9T/sbVSdz4TwsWzI04M7WALwIGEdqLhbalunq43zIMNl9lqw6y+qZuvBff/SXk1tePE0A62AFRdtBMJmPFnlV0n6HZA0MfAFYHVyskILAEoMmgHEtZLdfOo6gbqo1gKk3D90oQTC4UlAKcv2jgXzLwmeAv2B9omFxyfaSwUloCQqo+77DsxU22YLwWdBFSHvmMzSwBtmAs2PBqQhsW1XadtLjiz0HhEAtAAbZQLdnUajAa8QtpV4DskeQXgvE0b5YC8TgMJYXNVJ8B3VPYKuDJ022mUw0YxrgBtRhgdvu/EJBJw6+qk0yiHcmQBKJBoHxxwHU9jEgnAEqKNckDup0GEoBWA0yuNSSRAWwShkqNBhIBXivbBwVcMeQVoswDq+elqWA3Qyqdltw8OwVkAhQRtlMvrtTgC5ir6OsB3TeYVALSXnzuFWrOSowFJQA0QclT2HYhYAkKOwsjfNCgJ2vQHOEdilgBtJrBoX4Xn5YbTlgRfBgAsAdgHQm+DkBa5myKWfcjMW3znAMASAGJchWNPWKi4AbcS4zIE4ARLY2gHW4C2JG4HUhoqRRQ3FpwccXdAP6vFVwJb2AJAjFWQBph93z2ARSQA9+20s14Eq5WOvRMiASAkJaYBvqqnY94LsQAUFqG/+0kKZCrJt0JALADE/pIkFpKlb1EJADGzQgw4RU871AJAr3xTjH2Jjo1LkACAMrmbr4PvxsdHsACAi1PtiVFL6G+DLFEEANTdOH3RgSYBUp10t+9ENAEWbI5J/WwO6dd3xSUlugALlmfIV2qtIHDU9tzyVkJiAix4NTB46euBVYQNLtZS70TiAlpBFWn/NgApFAG2Yv8txk9guaQqoBfJBdAHWSMXQB9kjVwAfZA1/gJ/CVaakPorswAAAABJRU5ErkJggg==",
  properties: {
    colors: {
      dark: ["#a6adc8", "#9399b2", "#7f849c", "#6c7086", "#585b70", "#45475a", "#1e1e2e", "#181825", "#181825", "#11111b"],
    } as { [key: string]: string[] },
    other: {
      logoColor: "#89b4fa",
      positiveColor: "#a6e3a1",
      negativeColor: "#f38ba8",
      profit: "#cba6f7",
      userStatus: {
        [UserStatus.Online]: "#a6e3a1",
        [UserStatus.Invisible]: "#f38ba8",
        [UserStatus.Ingame]: "#cba6f7",
      },
      transactionType: {
        [TauriTypes.TransactionType.Purchase]: "#f38ba8",
        [TauriTypes.TransactionType.Sale]: "#a6e3a1",
        [TauriTypes.TransactionType.Trade]: "#89b4fa",
      },
      stockStatus: {
        [TauriTypes.StockStatus.Pending]: "#cba6f7",
        [TauriTypes.StockStatus.Live]: "#a6e3a1",
        [TauriTypes.StockStatus.ToLowProfit]: "#f9e2af",
        [TauriTypes.StockStatus.NoSellers]: "#fab387",
        [TauriTypes.StockStatus.NoBuyers]: "#f5c2e7",
        [TauriTypes.StockStatus.InActive]: "#eba0ac",
        [TauriTypes.StockStatus.SMALimit]: "#89b4fa",
        [TauriTypes.StockStatus.OrderLimit]: "#94e2d5",
        [TauriTypes.StockStatus.Overpriced]: "#f38ba8",
        [TauriTypes.StockStatus.Underpriced]: "#fab387",
        [TauriTypes.StockStatus.MaxPriceDrop]: "#f5c2e7",
      },
      alertType: {
        error: "#f38ba8",
        warning: "#fab387",
        info: "#89b4fa",
        success: "#a6e3a1",
      },
      itemType: {
        [TauriTypes.TransactionItemType.Item]: "#89b4fa",
        [TauriTypes.TransactionItemType.Riven]: "#cba6f7",
      },
      chartStyles: {
        total: {
          bgColor: "#1e1e2e",
          lastYearLineColor: "#74c7ec",
          currentYearLineColor: "#89b4fa",
        },
        today: {
          bgColor: "#1e1e2e",
          lineColor: "#b4befe",
        },
        lastDays: {
          bgColor: "#1e1e2e",
          lineColor: "#cba6f7",
        },
      },
      riven: {},
    },
  } as Record<string, any>,
};
