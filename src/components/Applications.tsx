import { Setting } from "../utils/types"

type props  = {
    apps: Setting[]
}

export default function Application({ apps }: props) {
    return (
        <table className="w-full">
          {/*Table header*/}
          <tr className="border-b border-neutral-300 py-3 text-neutral-500 text-left">
            <th>Name</th>
            {/* <th>Type</th> */}
            <th>Hotkey</th>
            <th>Mode</th>
            <th>Enabled</th>
          </tr>
          {apps.length > 0 && (
            <>
              {apps.map((app) => (
                <tr className="border-b border-neutral-300 px-4 p-4 hover:bg-neutral-50">
                  <td>{app.name}</td>
                  {/* <td>{app.app_type}</td> */}
                  <td>{app.hotkey ? app.hotkey : "Input Hotkey"}</td>
                  <td>{app.mode}</td>
                  <td>{app.enabled ? "True" : "False"}</td>
                </tr>
              ))}
            </>
          )}
        </table>
    )
}