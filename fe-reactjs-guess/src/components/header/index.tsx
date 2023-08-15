import { useSelector } from "react-redux";
import { Link, useNavigate } from "react-router-dom";
import Swal from "sweetalert2";
import withReactContent from "sweetalert2-react-content";
import { removeToken } from "../../redux/authSlice";
import { AppStateType, useAppDispatch } from "../../redux/store";

const MySwal = withReactContent(Swal);

export default function Header() {
  const authState = useSelector((state: AppStateType) => state.authState);
  const navigate = useNavigate();
  const dispatch = useAppDispatch();

  return (
    <>
      <header>
        <div className="d-flex flex-column flex-md-row align-items-center pb-3 mb-4 border-bottom">
          <Link
            to="/"
            className="d-flex align-items-center link-body-emphasis text-decoration-none"
          >
            <img src="enigma1.png" style={{ width: "40px", height: "40px" }} />
            <span className="fs-4">Number Guessing</span>
          </Link>

          <nav className="d-inline-flex mt-2 mt-md-0 ms-md-auto">
            <Link
              to="/about-us"
              className="me-3 py-2 link-body-emphasis text-decoration-none"
            >
              About Us
            </Link>

            {authState.user ? (
              <>
                <Link
                  to="/user"
                  className="me-3 py-2 link-body-emphasis text-decoration-none"
                >
                  <strong>{authState.user.fullname}</strong>
                </Link>
                <a
                  href="#"
                  className="py-2 link-body-emphasis text-decoration-none"
                  onClick={(e) => {
                    MySwal.fire({
                      title: <p>Çıkış yapmak istediğinize emin misiniz?</p>,
                      icon: "question",
                      showCancelButton: true,
                    }).then((result) => {
                      console.warn("🔥 | result:", result);

                      if (result.isConfirmed) {
                        dispatch(removeToken());
                        navigate("/");
                      }
                    });

                    e.preventDefault();
                  }}
                >
                  Logout
                </a>
              </>
            ) : (
              <>
                <Link
                  to="/auth/login"
                  className="me-3 py-2 link-body-emphasis text-decoration-none"
                >
                  Login
                </Link>
                <Link
                  to="/auth/register"
                  className="me-3 py-2 link-body-emphasis text-decoration-none"
                >
                  Register
                </Link>
              </>
            )}
          </nav>
        </div>
      </header>
    </>
  );
}
