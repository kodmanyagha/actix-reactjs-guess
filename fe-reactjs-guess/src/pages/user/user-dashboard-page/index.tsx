import Dropdown from "react-bootstrap/Dropdown";
import { useSelector } from "react-redux";
import { AppStateType } from "../../../features/state/store";

export default function UserDashboardPage() {
  const authState = useSelector((state: AppStateType) => state.authState);

  return (
    <>
      <main>
        <div
          className="d-flex flex-column flex-shrink-0 p-3 "
          style={{ width: "280px" }}
        >
          <ul className="nav nav-pills flex-column mb-auto">
            <li className="nav-item">
              <a href="#" className="nav-link active" aria-current="page">
                <svg className="bi me-2" width="16" height="16">
                </svg>
                Dashboard
              </a>
            </li>
            <li>
              <a href="#" className="nav-link link-dark">
                <svg className="bi me-2" width="16" height="16">
                  <use xlinkHref="#table" />
                </svg>
                Orders
              </a>
            </li>
            <li>
              <a href="#" className="nav-link link-dark">
                <svg className="bi me-2" width="16" height="16">
                  <use xlinkHref="#grid" />
                </svg>
                Products
              </a>
            </li>
            <li>
              <a href="#" className="nav-link link-dark">
                <svg className="bi me-2" width="16" height="16">
                  <use xlinkHref="people-circle" />
                </svg>
                Customers
              </a>
            </li>
          </ul>
          <hr />

          <Dropdown>
            <Dropdown.Toggle
              className="align-items-center w-100 text-decoration-none dropdown-toggle"
              variant="primary"
              id="userMenuDropdown"
            >
              <i className="fa-solid fa-user me-1" />

              <strong>{authState.user?.firstname + " " + authState.user?.lastname}</strong>
            </Dropdown.Toggle>

            <Dropdown.Menu>
              <Dropdown.Item href="#/action-1">Dashboard</Dropdown.Item>
              <Dropdown.Item href="#/action-2">My Projects</Dropdown.Item>
              <Dropdown.Item href="#/action-3">Settings</Dropdown.Item>
              <Dropdown.Divider />
              <Dropdown.Item href="javascript:void(0)">Logout</Dropdown.Item>
            </Dropdown.Menu>
          </Dropdown>
        </div>
      </main>
    </>
  );
}
