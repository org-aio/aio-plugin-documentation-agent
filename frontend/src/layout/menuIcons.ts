import { resolveIcon } from '@/components/Icon/icons'
import {
  Document,
  Folder,
  Grid,
  HomeFilled,
  Key,
  Menu,
  Setting,
  Tickets,
  User,
  Tools
} from '@element-plus/icons-vue'
const icons = {
  home: HomeFilled,
  table: Grid,
  tree: Tickets,
  page: Document,
  user: User,
  key: Key,
  menu: Menu,
  setting: Setting,
  folder: Folder,
  system: Setting,
  infra: Tools,
  examples: Grid
}
export const iconFor = (name: string) => icons[name as keyof typeof icons] || resolveIcon(name)
