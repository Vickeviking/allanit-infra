export default [
  {
    id: "seg_brf_all",
    name: "Alla BRF-kunder",
    filters: { tags: ["brf"] },
    count_hint: 3
  },
  {
    id: "seg_vinterberedskap",
    name: "Vinterberedskap",
    filters: { tags: ["vinter", "drift"] },
    count_hint: 2
  },
  {
    id: "seg_miljorum",
    name: "Miljörumskunder",
    filters: { tags: ["miljörum"] },
    count_hint: 2
  },
  {
    id: "seg_garage",
    name: "Garagekunder",
    filters: { tags: ["garage"] },
    count_hint: 2
  },
  {
    id: "seg_vip",
    name: "VIP-kunder",
    filters: { tags: ["vip"] },
    count_hint: 1
  }
]
