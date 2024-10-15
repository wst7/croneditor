import { Outlet } from "react-router-dom";
import { ConfigProvider } from "antd";

function App() {
  return (
    <ConfigProvider
      theme={{
        components: {
          
          Tag: {
            fontSize: 12,
            lineHeightSM: 1.2,
            marginXS: 2,
          },
          Divider: {
            marginLG: 10,
          },
        },
      }}
    >
      <div className="h-screen border-t"><Outlet /></div> 
    </ConfigProvider>
  );
}

export default App;
